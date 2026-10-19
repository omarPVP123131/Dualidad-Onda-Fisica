//! Lotes de geometría 3D reutilizables.
//!
//! En macroquad cada `draw_line_3d`, `draw_cube` o `draw_sphere` sube su
//! propia geometría al GPU: dibujar cientos de líneas por frame cuesta
//! muchísimo (sobre todo con render por software). Este módulo acumula
//! triángulos en mallas grandes (≤4 800 índices, el tope seguro de
//! macroquad) y las dibuja en unas pocas llamadas. Los búferes se
//! reutilizan entre frames para no asignar memoria en el bucle de dibujo.

use macroquad::prelude::*;

/// Índices máximos por malla (el tope de macroquad es 5 000).
pub const MAX_INDICES: usize = 4_800;
/// Índices que ocupa un quad (dos triángulos).
pub const QUAD_INDICES: usize = 6;
/// Índices que ocupa un disco de 8 gajos.
const DISC_INDICES: usize = 24;
/// Gajos del disco barato que sustituye a las esferas de las partículas.
const DISC_SEGMENTS: usize = 8;

fn blank_mesh() -> Mesh {
    Mesh {
        vertices: Vec::new(),
        indices: Vec::new(),
        texture: None,
    }
}

/// Acumulador de triángulos en mallas de tamaño seguro.
#[derive(Default)]
pub struct MeshBatch {
    meshes: Vec<Mesh>,
    current: usize,
}

impl MeshBatch {
    /// Vacía el lote conservando capacidad y búferes.
    pub fn clear(&mut self) {
        for mesh in &mut self.meshes {
            mesh.vertices.clear();
            mesh.indices.clear();
        }
        self.current = 0;
    }

    /// Mallas con contenido en este frame (las vacías quedan fuera).
    pub fn meshes(&self) -> &[Mesh] {
        if self.meshes.is_empty() || self.meshes[0].indices.is_empty() {
            &[]
        } else {
            &self.meshes[..self.current + 1]
        }
    }

    /// Mallas construidas (para cachear escena estática).
    pub fn into_meshes(mut self) -> Vec<Mesh> {
        let used = self.meshes().len();
        self.meshes.truncate(used);
        self.meshes
    }

    fn ensure_room(&mut self, needed: usize) {
        if self.current >= self.meshes.len() {
            self.meshes.push(blank_mesh());
        } else if self.meshes[self.current].indices.len() + needed > MAX_INDICES {
            self.current += 1;
            if self.current >= self.meshes.len() {
                self.meshes.push(blank_mesh());
            }
        }
    }

    /// Un quad con los vértices en orden (el material por defecto no
    /// descarta caras, así que ambos lados se ven).
    pub fn push_quad(&mut self, p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, color: Color) {
        self.ensure_room(QUAD_INDICES);
        let mesh = &mut self.meshes[self.current];
        let base = mesh.vertices.len() as u16;
        for p in [p0, p1, p2, p3] {
            mesh.vertices
                .push(Vertex::new(p.x, p.y, p.z, 0.0, 0.0, color));
        }
        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    /// Cubo completo (6 caras), mismo orden de caras que `draw_cube`.
    pub fn push_cube(&mut self, center: Vec3, size: Vec3, color: Color) {
        let (x, y, z) = (center.x, center.y, center.z);
        let (hw, hh, hl) = (size.x / 2.0, size.y / 2.0, size.z / 2.0);
        self.push_quad(
            vec3(x - hw, y - hh, z + hl),
            vec3(x + hw, y - hh, z + hl),
            vec3(x + hw, y + hh, z + hl),
            vec3(x - hw, y + hh, z + hl),
            color,
        );
        self.push_quad(
            vec3(x - hw, y - hh, z - hl),
            vec3(x + hw, y - hh, z - hl),
            vec3(x + hw, y + hh, z - hl),
            vec3(x - hw, y + hh, z - hl),
            color,
        );
        self.push_quad(
            vec3(x - hw, y + hh, z - hl),
            vec3(x - hw, y + hh, z + hl),
            vec3(x + hw, y + hh, z + hl),
            vec3(x + hw, y + hh, z - hl),
            color,
        );
        self.push_quad(
            vec3(x - hw, y - hh, z - hl),
            vec3(x - hw, y - hh, z + hl),
            vec3(x + hw, y - hh, z + hl),
            vec3(x + hw, y - hh, z - hl),
            color,
        );
        self.push_quad(
            vec3(x - hw, y - hh, z + hl),
            vec3(x - hw, y - hh, z - hl),
            vec3(x - hw, y + hh, z - hl),
            vec3(x - hw, y + hh, z + hl),
            color,
        );
        self.push_quad(
            vec3(x + hw, y - hh, z + hl),
            vec3(x + hw, y - hh, z - hl),
            vec3(x + hw, y + hh, z - hl),
            vec3(x + hw, y + hh, z + hl),
            color,
        );
    }

    /// Aristas de una caja como 12 barras finas (sustituye a
    /// `draw_cube_wires`, que gastaba una llamada por lado).
    pub fn push_box_wires(&mut self, center: Vec3, size: Vec3, thickness: f32, color: Color) {
        let (x, y, z) = (center.x, center.y, center.z);
        let (hw, hh, hl) = (size.x / 2.0, size.y / 2.0, size.z / 2.0);
        let t = thickness.max(0.004);
        for sy in [-hh, hh] {
            for sz in [-hl, hl] {
                self.push_cube(vec3(x, y + sy, z + sz), vec3(size.x + t, t, t), color);
            }
        }
        for sx in [-hw, hw] {
            for sz in [-hl, hl] {
                self.push_cube(vec3(x + sx, y, z + sz), vec3(t, size.y + t, t), color);
            }
        }
        for sx in [-hw, hw] {
            for sy in [-hh, hh] {
                self.push_cube(vec3(x + sx, y + sy, z), vec3(t, t, size.z + t), color);
            }
        }
    }

    /// Cinta plana a lo largo de `p0 → p1` contenida en el plano de
    /// `normal` (sustituye a las líneas del suelo y de la pantalla).
    pub fn push_band(&mut self, p0: Vec3, p1: Vec3, normal: Vec3, half: f32, color: Color) {
        let dir = (p1 - p0).normalize_or_zero();
        let side = dir.cross(normal).normalize_or_zero() * half;
        if side == Vec3::ZERO {
            return;
        }
        self.push_quad(p0 - side, p0 + side, p1 + side, p1 - side, color);
    }

    /// Disco orientado a la cámara (silueta barata de una esfera).
    pub fn push_disc(&mut self, center: Vec3, right: Vec3, up: Vec3, radius: f32, color: Color) {
        self.ensure_room(DISC_INDICES);
        let mesh = &mut self.meshes[self.current];
        let base = mesh.vertices.len() as u16;
        mesh.vertices
            .push(Vertex::new(center.x, center.y, center.z, 0.5, 0.5, color));
        // Radio corregido para que el octógono inscriba la circunferencia.
        let r = radius / (std::f32::consts::PI / DISC_SEGMENTS as f32).cos();
        for s in 0..DISC_SEGMENTS {
            let a = s as f32 / DISC_SEGMENTS as f32 * std::f32::consts::TAU;
            let p = center + right * (a.cos() * r) + up * (a.sin() * r);
            mesh.vertices
                .push(Vertex::new(p.x, p.y, p.z, a.cos(), a.sin(), color));
        }
        for s in 0..DISC_SEGMENTS as u16 {
            let rim = base + 1 + s;
            let next = base + 1 + (s + 1) % DISC_SEGMENTS as u16;
            mesh.indices.extend_from_slice(&[base, rim, next]);
        }
    }

    /// Dibuja las mallas acumuladas (una llamada por malla).
    pub fn draw(&self) {
        for mesh in self.meshes() {
            draw_mesh(mesh);
        }
    }
}

/// Cintas finas orientadas a la cámara: sustituyen a `draw_line_3d`, que
/// gasta una subida de geometría por cada línea dibujada.
#[derive(Default)]
pub struct LineBatch {
    batch: MeshBatch,
    forward: Vec3,
}

impl LineBatch {
    /// Prepara un frame nuevo: fija la dirección de vista (para orientar
    /// las cintas) y conserva la capacidad reservada.
    pub fn begin(&mut self, forward: Vec3) {
        self.batch.clear();
        self.forward = forward.normalize_or_zero();
    }

    /// Convierte una polilínea en cintas continuas.
    pub fn polyline(&mut self, points: &[Vec3], half_width: f32, color: Color) {
        for pair in points.windows(2) {
            self.segment(pair[0], pair[1], half_width, color);
        }
    }

    /// Una cinta de ancho `2·half_width` que siempre mira a la cámara.
    pub fn segment(&mut self, p0: Vec3, p1: Vec3, half_width: f32, color: Color) {
        let delta = p1 - p0;
        if delta.length_squared() < 1e-10 {
            return;
        }
        let dir = delta.normalize();
        let mut side = dir.cross(self.forward).normalize_or_zero();
        if side == Vec3::ZERO {
            // Segmento en la dirección de la vista: cualquier perpendicular sirve.
            side = dir.cross(Vec3::Y).normalize_or_zero();
            if side == Vec3::ZERO {
                side = Vec3::X;
            }
        }
        let side = side * half_width;
        self.batch
            .push_quad(p0 - side, p0 + side, p1 + side, p1 - side, color);
    }

    /// Dibuja las cintas acumuladas.
    pub fn draw(&self) {
        self.batch.draw();
    }
}

/// Base ortonormal de la cámara para orientar cintas y discos.
#[derive(Clone, Copy, Debug)]
pub struct ViewBasis {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl ViewBasis {
    pub fn new(position: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - position).normalize_or_zero();
        let right = forward.cross(up).normalize_or_zero();
        Self {
            forward,
            right,
            up: right.cross(forward),
        }
    }

    pub fn of(camera: &Camera3D) -> Self {
        Self::new(camera.position, camera.target, camera.up)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn los_lotes_respetan_el_tope_de_indices() {
        let mut batch = MeshBatch::default();
        let total = 1_000;
        for i in 0..total {
            let x = i as f32 * 0.001;
            batch.push_quad(
                vec3(x, 0.0, 0.0),
                vec3(x, 1.0, 0.0),
                vec3(x, 1.0, 1.0),
                vec3(x, 0.0, 1.0),
                WHITE,
            );
        }
        let meshes = batch.meshes();
        assert!(meshes.len() > 1, "debe repartirse en varias mallas");
        for mesh in meshes {
            assert!(
                mesh.indices.len() <= MAX_INDICES,
                "índices={}",
                mesh.indices.len()
            );
            assert!(mesh
                .indices
                .iter()
                .all(|&i| (i as usize) < mesh.vertices.len()));
        }
        let sum: usize = meshes.iter().map(|m| m.indices.len()).sum();
        assert_eq!(sum, total * QUAD_INDICES);
    }

    #[test]
    fn clear_conserva_los_buferes() {
        let mut batch = MeshBatch::default();
        for _ in 0..900 {
            batch.push_cube(Vec3::ZERO, Vec3::ONE, WHITE);
        }
        let reserved = batch.meshes.len();
        batch.clear();
        assert_eq!(batch.meshes().len(), 0, "vacío tras limpiar");
        batch.push_quad(Vec3::ZERO, Vec3::X, Vec3::ONE, Vec3::Y, WHITE);
        assert_eq!(batch.meshes().len(), 1);
        assert!(
            batch.meshes.len() >= reserved,
            "las mallas reservadas se reutilizan"
        );
    }

    #[test]
    fn las_primitivas_declaran_sus_indices() {
        let mut batch = MeshBatch::default();
        batch.push_cube(Vec3::ZERO, Vec3::ONE, WHITE);
        assert_eq!(batch.meshes()[0].indices.len(), 6 * QUAD_INDICES);
        batch.clear();
        batch.push_box_wires(Vec3::ZERO, Vec3::ONE, 0.01, WHITE);
        assert_eq!(batch.meshes()[0].indices.len(), 12 * 6 * QUAD_INDICES);
        batch.clear();
        batch.push_disc(Vec3::ZERO, Vec3::X, Vec3::Y, 0.1, WHITE);
        assert_eq!(batch.meshes()[0].indices.len(), DISC_INDICES);
        batch.clear();
        batch.push_band(Vec3::ZERO, Vec3::X, Vec3::Y, 0.01, WHITE);
        assert_eq!(batch.meshes()[0].indices.len(), QUAD_INDICES);
        batch.clear();
        batch.push_band(Vec3::ZERO, Vec3::ZERO, Vec3::Y, 0.01, WHITE);
        assert_eq!(batch.meshes().len(), 0, "una banda degenerada no dibuja");
    }

    #[test]
    fn las_lineas_se_convierten_en_cintas() {
        let mut lines = LineBatch::default();
        lines.begin(vec3(0.0, 0.0, -1.0));
        lines.polyline(
            &[
                vec3(0.0, 0.0, 0.0),
                vec3(1.0, 0.0, 0.0),
                vec3(1.0, 1.0, 0.0),
            ],
            0.01,
            WHITE,
        );
        let indices: usize = lines.batch.meshes().iter().map(|m| m.indices.len()).sum();
        assert_eq!(indices, 2 * QUAD_INDICES, "dos segmentos = dos quads");
        // Un segmento en la dirección de la vista también genera cinta
        // (usa una perpendicular de respaldo).
        lines.begin(vec3(0.0, 0.0, -1.0));
        lines.segment(Vec3::ZERO, vec3(0.0, 0.0, -1.0), 0.01, WHITE);
        assert_eq!(lines.batch.meshes()[0].indices.len(), QUAD_INDICES);
    }

    #[test]
    fn la_base_de_vista_es_ortonormal() {
        let v = ViewBasis::new(vec3(0.0, 0.0, 10.0), Vec3::ZERO, Vec3::Y);
        assert!((v.forward + vec3(0.0, 0.0, 1.0)).length() < 1e-2);
        assert!(v.forward.dot(v.right).abs() < 1e-5);
        assert!(v.right.dot(v.up).abs() < 1e-5);
        assert!((v.up - Vec3::Y).length() < 1e-2);
    }
}
