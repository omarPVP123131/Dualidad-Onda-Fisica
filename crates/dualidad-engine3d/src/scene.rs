//! La escena del laboratorio: banco óptico, fuente, barrera con dos
//! rendijas, sensores del detector y pantalla.
//!
//! Las distancias transversales son **esquemáticas**: a escala real
//! (rendijas de µm frente a una pantalla de 40 mm) nada se vería. La
//! separación y el ancho de las rendijas se exagera con una ley lineal
//! saturada, solo para representar, nunca para medir.
//!
//! La geometría estática se agrupa en mallas (`build_lab_meshes`), que el
//! renderer cachea: antes cada cubo, línea o esfera subía su propia
//! geometría al GPU en cada frame (~300 llamadas); ahora son unas pocas.
//! Solo quedan sueltas las piezas redondas (esferas y cilindros), pocas
//! y no aptas para teselar a mano.

use crate::batch::MeshBatch;
use crate::color;
use crate::layout;
use dualidad_core::SimulationParams;
use macroquad::prelude::*;

/// Centros visuales de las `slit_count` rendijas en X del mundo.
pub fn slit_centers_x(params: &SimulationParams) -> Vec<f32> {
    let n = params.slit_count.max(1);
    let sep = visual_separation(params);
    let mid = (n as f32 - 1.0) / 2.0;
    (0..n).map(|i| (i as f32 - mid) * sep).collect()
}

/// Separación visual entre rendijas (exagerada, saturada).
pub fn visual_separation(params: &SimulationParams) -> f32 {
    (params.slit_separation_m as f32 * 8_000.0).clamp(0.3, 2.4)
}

/// Ancho visual de cada rendija (exagerado, saturado).
pub fn visual_slit_width(params: &SimulationParams) -> f32 {
    (params.slit_width_m as f32 * 9_000.0).clamp(0.07, 0.36)
}

/// Bloques de pared (centro, ancho) que quedan al abrir las dos rendijas.
fn wall_blocks(params: &SimulationParams) -> Vec<(f32, f32)> {
    let w = layout::SCREEN_WIDTH - 0.4;
    let s = visual_slit_width(params) / 2.0;
    let mut cuts: Vec<f32> = vec![-w / 2.0];
    for c in slit_centers_x(params) {
        cuts.push(c - s);
        cuts.push(c + s);
    }
    cuts.push(w / 2.0);
    let mut blocks = Vec::new();
    for pair in cuts.chunks(2) {
        if pair.len() == 2 {
            let width = pair[1] - pair[0];
            if width > 0.01 {
                blocks.push(((pair[0] + pair[1]) / 2.0, width));
            }
        }
    }
    blocks
}

/// Clave de caché de la geometría estática del laboratorio: cambia si
/// cambia la física representada o el detalle ambiental. El detector no
/// entra: su luz de estado va en `draw_lab_extras`, fuera de las mallas.
#[derive(PartialEq, Debug)]
pub struct LabKey {
    wavelength: u64,
    separation: u64,
    width: u64,
    slits: usize,
    ambient: bool,
}

impl LabKey {
    pub fn of(params: &SimulationParams, ambient: bool) -> Self {
        Self {
            wavelength: params.wavelength_m.to_bits(),
            separation: params.slit_separation_m.to_bits(),
            width: params.slit_width_m.to_bits(),
            slits: params.slit_count,
            ambient,
        }
    }
}

/// Construye la escena estática del laboratorio agrupada en mallas
/// (una llamada de dibujo por malla; la caché la lleva el renderer).
pub fn build_lab_meshes(params: &SimulationParams, ambient: bool) -> Vec<Mesh> {
    let axis_y = layout::OPTICAL_AXIS_Y;
    let wave = color::wave_color(params.wavelength_m, 1.0);
    let zf = layout::SCREEN_Z - 0.16;
    let (hw, hh) = (layout::SCREEN_WIDTH / 2.0, layout::SCREEN_HEIGHT / 2.0);
    let mut b = MeshBatch::default();

    // Estrellas de polvo lejanas sobre el fondo (deterministas).
    if ambient {
        let mut seed = 0x2545_u32;
        let mut rnd = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / 16_777_216.0
        };
        for _ in 0..80 {
            let x = -10.0 + rnd() * 20.0;
            let y = 0.4 + rnd() * 6.5;
            b.push_cube(
                vec3(x, y, 8.45),
                vec3(0.035, 0.035, 0.01),
                Color::new(0.6, 0.7, 0.9, 0.28),
            );
        }
        // Resplandor del haz sobre el suelo.
        let glow = color::wave_color(params.wavelength_m, 0.045);
        b.push_quad(
            vec3(-4.75, 0.018, -6.45),
            vec3(4.75, 0.018, -6.45),
            vec3(4.75, 0.018, 7.05),
            vec3(-4.75, 0.018, 7.05),
            glow,
        );
    }

    // --- Fondo de la sala y suelo ---
    b.push_cube(
        vec3(0.0, 3.4, 8.6),
        vec3(22.0, 9.0, 0.2),
        Color::new(0.050, 0.066, 0.105, 1.0),
    );
    b.push_cube(vec3(0.0, -0.07, 0.4), vec3(15.0, 0.14, 15.5), color::FLOOR);
    // Sombras falsas suaves bajo cada componente (sin motor de luz).
    let sombra = Color::new(0.0, 0.0, 0.0, 0.35);
    for (sx, sz, w, d) in [
        (0.0, layout::SOURCE_Z, 1.1, 1.2),
        (0.0, layout::SLITS_Z, 8.2, 0.8),
        (0.0, layout::SCREEN_Z, 8.8, 0.9),
    ] {
        b.push_cube(vec3(sx, 0.012, sz), vec3(w, 0.012, d), sombra);
    }
    for i in -7..=7 {
        let x = i as f32;
        b.push_band(
            vec3(x, 0.005, -6.8),
            vec3(x, 0.005, 7.6),
            Vec3::Y,
            0.008,
            color::GRID,
        );
        let z = i as f32;
        b.push_band(
            vec3(-7.2, 0.005, z),
            vec3(7.2, 0.005, z),
            Vec3::Y,
            0.008,
            color::GRID,
        );
    }

    // --- Banco óptico: dos rieles, travesaños y regla graduada ---
    for rx in [-0.38, 0.38] {
        b.push_cube(vec3(rx, 0.10, 0.3), vec3(0.16, 0.18, 13.6), color::METAL);
    }
    for i in 0..7 {
        let z = -6.0 + i as f32 * 2.1;
        b.push_cube(vec3(0.0, 0.055, z), vec3(1.15, 0.09, 0.30), color::WALL);
        b.push_cube(
            vec3(0.62, 0.16, z),
            vec3(0.16, 0.03, 0.03),
            color::WALL_EDGE,
        );
    }
    // Placas base de cada componente.
    for z in [layout::SOURCE_Z, layout::SLITS_Z, layout::SCREEN_Z] {
        b.push_cube(vec3(0.0, 0.23, z), vec3(1.0, 0.08, 0.9), color::METAL);
    }

    // --- Fuente ---
    let source = vec3(0.0, axis_y, layout::SOURCE_Z);
    b.push_cube(source, vec3(0.42, 0.42, 0.52), color::WALL);
    b.push_box_wires(source, vec3(0.42, 0.42, 0.52), 0.012, color::WALL_EDGE);
    // Anillos de emisión (cintas planas en el plano XY de la fuente).
    for (r, a) in [(0.17, 0.55), (0.25, 0.30)] {
        let c = Color::new(wave.r, wave.g, wave.b, a);
        for s in 0..30 {
            let a0 = s as f32 / 30.0 * std::f32::consts::TAU;
            let a1 = (s + 1) as f32 / 30.0 * std::f32::consts::TAU;
            let p0 = source + vec3(a0.cos() * r, a0.sin() * r, 0.37);
            let p1 = source + vec3(a1.cos() * r, a1.sin() * r, 0.37);
            let s0 = vec3(a0.cos(), a0.sin(), 0.0) * 0.006;
            let s1 = vec3(a1.cos(), a1.sin(), 0.0) * 0.006;
            b.push_quad(p0 - s0, p0 + s0, p1 + s1, p1 - s1, c);
        }
    }
    // Haz incidente (guía visual fina, no una trayectoria medida).
    b.push_cube(
        vec3(
            0.0,
            axis_y,
            (layout::SOURCE_Z + layout::SLITS_Z) / 2.0 + 0.2,
        ),
        vec3(0.045, 0.045, layout::SLITS_Z - layout::SOURCE_Z - 0.4),
        color::wave_color(params.wavelength_m, 0.35),
    );

    // --- Barrera con dos rendijas ---
    let blocks = wall_blocks(params);
    for &(cx, width) in &blocks {
        let size = vec3(width, layout::SCREEN_HEIGHT, 0.16);
        let pos = vec3(cx, axis_y, layout::SLITS_Z);
        b.push_cube(pos, size, color::WALL);
        b.push_box_wires(pos, size, 0.012, color::WALL_EDGE);
    }
    // Degradado frontal fingido (sin motor de luz: 5 bandas por bloque).
    for &(cx, width) in &blocks {
        for band in 0..5 {
            let f = band as f32 / 4.0;
            let band_h = layout::SCREEN_HEIGHT / 5.0;
            let y = axis_y - layout::SCREEN_HEIGHT / 2.0 + band_h * (band as f32 + 0.5);
            let shade = Color::new(0.10 + 0.07 * f, 0.12 + 0.07 * f, 0.17 + 0.09 * f, 1.0);
            b.push_cube(
                vec3(cx, y, layout::SLITS_Z - 0.095),
                vec3(width, band_h, 0.012),
                shade,
            );
        }
    }
    b.push_cube(
        vec3(
            0.0,
            axis_y + layout::SCREEN_HEIGHT / 2.0 + 0.05,
            layout::SLITS_Z,
        ),
        vec3(layout::SCREEN_WIDTH - 0.2, 0.10, 0.30),
        color::METAL,
    );
    // Marco luminoso mínimo alrededor de cada rendija (sin él, las
    // rendijas a esta escala son invisibles y la barrera parece ciega).
    {
        let glow = color::wave_color(params.wavelength_m, 0.95);
        let w = visual_slit_width(params);
        let h = layout::SCREEN_HEIGHT - 0.7;
        let z = layout::SLITS_Z - 0.11;
        for cx in slit_centers_x(params) {
            for side in [-1.0f32, 1.0] {
                b.push_cube(
                    vec3(cx + side * (w / 2.0 + 0.015), axis_y, z),
                    vec3(0.03, h, 0.03),
                    glow,
                );
            }
        }
    }
    // Patas de la barrera.
    for x in [-3.3, 3.3] {
        b.push_cube(
            vec3(
                x,
                axis_y - layout::SCREEN_HEIGHT / 2.0 - 0.35,
                layout::SLITS_Z,
            ),
            vec3(0.18, 0.7, 0.3),
            color::METAL,
        );
    }
    // Sensores del detector: solo el cuerpo va en mallas (lente y luz de
    // estado son cilindro y esfera, van en `draw_lab_extras`).
    for cx in slit_centers_x(params) {
        let top = axis_y + layout::SCREEN_HEIGHT / 2.0;
        let body = vec3(cx, top + 0.16, layout::SLITS_Z);
        b.push_cube(body, vec3(0.34, 0.2, 0.34), color::METAL);
    }

    // --- Módulo electrónico del detector, sobre la barrera ---
    // (Integrado en el aparato: una caja suelta en el suelo estorbaba.)
    let centers = slit_centers_x(params);
    let span = centers.last().copied().unwrap_or(0.0) - centers.first().copied().unwrap_or(0.0);
    let top_y = axis_y + layout::SCREEN_HEIGHT / 2.0 + 0.34;
    b.push_cube(
        vec3(0.0, top_y, layout::SLITS_Z),
        vec3(span + 1.1, 0.16, 0.4),
        color::METAL,
    );

    // --- Pantalla de detección ---
    let screen_pos = vec3(0.0, axis_y, layout::SCREEN_Z);
    let frame_size = vec3(
        layout::SCREEN_WIDTH + 0.35,
        layout::SCREEN_HEIGHT + 0.35,
        0.16,
    );
    b.push_cube(screen_pos, frame_size, color::WALL);
    b.push_cube(
        screen_pos - vec3(0.0, 0.0, 0.10),
        vec3(layout::SCREEN_WIDTH, layout::SCREEN_HEIGHT, 0.05),
        color::SCREEN,
    );
    b.push_box_wires(screen_pos, frame_size, 0.012, color::WALL_EDGE);
    // Marco luminoso y lineas de escaneo de la pantalla.
    {
        let glow = color::wave_color(params.wavelength_m, 0.5);
        for (a, c) in [
            (vec3(-hw, axis_y - hh, zf), vec3(hw, axis_y - hh, zf)),
            (vec3(hw, axis_y - hh, zf), vec3(hw, axis_y + hh, zf)),
            (vec3(hw, axis_y + hh, zf), vec3(-hw, axis_y + hh, zf)),
            (vec3(-hw, axis_y + hh, zf), vec3(-hw, axis_y - hh, zf)),
        ] {
            b.push_band(a, c, Vec3::Z, 0.006, glow);
        }
        for i in 1..if ambient { 9 } else { 1 } {
            let y = axis_y - hh + layout::SCREEN_HEIGHT * i as f32 / 9.0;
            b.push_band(
                vec3(-hw, y, zf),
                vec3(hw, y, zf),
                Vec3::Z,
                0.005,
                Color::new(1.0, 1.0, 1.0, 0.045),
            );
        }
    }
    b.push_cube(
        vec3(
            0.0,
            axis_y + layout::SCREEN_HEIGHT / 2.0 + 0.22,
            layout::SCREEN_Z,
        ),
        vec3(layout::SCREEN_WIDTH + 0.1, 0.10, 0.30),
        color::METAL,
    );
    // Patas de la pantalla.
    for x in [-3.3, 3.3] {
        b.push_cube(
            vec3(
                x,
                axis_y - layout::SCREEN_HEIGHT / 2.0 - 0.35,
                layout::SCREEN_Z,
            ),
            vec3(0.18, 0.7, 0.3),
            color::METAL,
        );
    }
    // Cruz central tenue sobre la pantalla (referencia del eje óptico).
    let cross_color = Color::new(0.25, 0.30, 0.40, 1.0);
    let zc = layout::SCREEN_Z - 0.14;
    b.push_band(
        vec3(-hw, axis_y, zc),
        vec3(hw, axis_y, zc),
        Vec3::Z,
        0.006,
        cross_color,
    );
    b.push_band(
        vec3(0.0, axis_y - hh, zc),
        vec3(0.0, axis_y + hh, zc),
        Vec3::Z,
        0.006,
        cross_color,
    );

    b.into_meshes()
}

/// Piezas redondas que quedan fuera de las mallas estáticas: pocas
/// esferas y cilindros que no merecen teselarse a mano.
pub fn draw_lab_extras(params: &SimulationParams, observe: bool) {
    let axis_y = layout::OPTICAL_AXIS_Y;
    let wave = color::wave_color(params.wavelength_m, 1.0);
    let status = if observe {
        color::SENSOR_ON
    } else {
        color::SENSOR_OFF
    };
    let source = vec3(0.0, axis_y, layout::SOURCE_Z);
    // Teselación reducida: a su tamaño en pantalla es indistinguible.
    let sphere = || DrawSphereParams {
        rings: 8,
        slices: 8,
        ..Default::default()
    };
    let cylinder = || DrawCylinderParams {
        sides: 10,
        ..Default::default()
    };
    draw_cylinder_ex(
        vec3(0.0, axis_y / 2.0, layout::SOURCE_Z),
        0.07,
        0.10,
        axis_y,
        None,
        color::METAL,
        cylinder(),
    );
    draw_sphere_ex(source + vec3(0.0, 0.0, 0.34), 0.11, None, wave, sphere());
    for cx in slit_centers_x(params) {
        let top = axis_y + layout::SCREEN_HEIGHT / 2.0;
        let body = vec3(cx, top + 0.16, layout::SLITS_Z);
        draw_cylinder_ex(
            body - vec3(0.0, 0.05, 0.24),
            0.055,
            0.055,
            0.16,
            None,
            color::WALL_EDGE,
            cylinder(),
        );
        draw_sphere_ex(body + vec3(0.0, 0.14, 0.0), 0.045, None, status, sphere());
    }
    let top_y = axis_y + layout::SCREEN_HEIGHT / 2.0 + 0.34;
    for dx in [-0.18f32, 0.18] {
        draw_sphere_ex(
            vec3(dx, top_y + 0.12, layout::SLITS_Z),
            0.045,
            None,
            status,
            sphere(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn siempre_quedan_tres_bloques_de_pared() {
        let p = SimulationParams::default();
        assert_eq!(wall_blocks(&p).len(), 3);
    }

    #[test]
    fn las_rendijas_son_simetricas() {
        let p = SimulationParams::default();
        let c = slit_centers_x(&p);
        assert_eq!(c.len(), 2);
        assert!((c[0] + c[1]).abs() < 1e-6 && c[0] < 0.0 && c[1] > 0.0);
        let mut p4 = p;
        p4.slit_count = 4;
        let c4 = slit_centers_x(&p4);
        assert_eq!(c4.len(), 4);
        assert!((c4[0] + c4[3]).abs() < 1e-6);
    }

    #[test]
    fn la_escena_por_lotes_respeta_el_tope_de_indices() {
        let p = SimulationParams::default();
        let meshes = build_lab_meshes(&p, true);
        assert!(!meshes.is_empty(), "el laboratorio debe producir mallas");
        for mesh in &meshes {
            assert!(
                mesh.indices.len() <= 5_000,
                "índices={}",
                mesh.indices.len()
            );
            assert!(mesh
                .indices
                .iter()
                .all(|&i| (i as usize) < mesh.vertices.len()));
        }
        // La clave no cambia si nada físico ni visual cambia.
        assert_eq!(LabKey::of(&p, true), LabKey::of(&p, true));
        let mut q = p;
        q.wavelength_m += 1e-9;
        assert_ne!(LabKey::of(&p, true), LabKey::of(&q, true));
    }
}
