//! Impactos sobre la pantalla 3D como mallas por lotes con anillo.
//!
//! Dibujar esfera por punto (hasta 40 000) sería inviable frame a frame.
//! Cada impacto es un quad diminuto dentro de mallas de `QUADS_PER_MESH`
//! quads y la caché es **incremental**: los impactos nuevos se escriben en
//! un anillo de ranuras fijas (sin reconstruir nada) y macroquad reenvía
//! la geometría al GPU al dibujar. La ventana deslizante del núcleo puede
//! recortar por delante sin tocar la caché: el anillo ya guarda los
//! impactos más recientes. Solo se repinta entero si cambia el color (λ),
//! el tope por nivel de calidad o hay un reinicio.

use crate::layout;
use dualidad_core::Simulation;
use macroquad::prelude::*;

/// 800 quads = 4 800 índices: bajo el límite por geometría de macroquad.
const QUADS_PER_MESH: usize = 800;
const QUAD: f32 = 0.030;

#[derive(Default)]
pub struct ImpactsCache {
    meshes: Vec<Mesh>,
    halo_meshes: Vec<Mesh>,
    /// Capacidad del anillo en quads (= tope de dibujo del nivel actual).
    slots: usize,
    /// ¿Se dibuja el halo? (nivel Ligera no).
    halo: bool,
    /// Ranuras escritas desde el arranque (tope: `slots`).
    filled: usize,
    /// Siguiente ranura del anillo.
    next: usize,
    built_total: u64,
    built_color_bits: u32,
}

impl ImpactsCache {
    pub fn draw(&mut self, sim: &Simulation, draw_cap: usize, halo_enabled: bool) {
        self.update(sim, draw_cap, halo_enabled);
        let used = self.filled.div_ceil(QUADS_PER_MESH);
        for mesh in self.halo_meshes.iter().take(used) {
            draw_mesh(mesh);
        }
        for mesh in self.meshes.iter().take(used) {
            draw_mesh(mesh);
        }
    }

    /// Sincroniza la caché con los impactos del núcleo (sin GL: testeable).
    fn update(&mut self, sim: &Simulation, draw_cap: usize, halo_enabled: bool) {
        let rgb = dualidad_core::wavelength_to_rgb(sim.params.wavelength_m);
        let color_bits = (rgb.r.to_bits() << 16) ^ rgb.g.to_bits() ^ rgb.b.to_bits();
        let count = sim.impacts().len();
        let total = sim.total();
        let color = crate::color::wave_color(sim.params.wavelength_m, 0.95);
        let halo = crate::color::wave_color(sim.params.wavelength_m, 0.16);
        let capacity_changed = self.slots != draw_cap || self.halo != halo_enabled;
        let reset = total < self.built_total;
        if capacity_changed || reset {
            self.slots = draw_cap;
            self.halo = halo_enabled;
            self.meshes = blank_meshes(draw_cap);
            self.halo_meshes = if halo_enabled {
                blank_meshes(draw_cap)
            } else {
                Vec::new()
            };
            // Repinta lo más reciente que quepa (tras un reinicio queda limpio).
            let n = count.min(self.slots);
            for (slot, impact) in sim.impacts()[count - n..].iter().enumerate() {
                self.write_slot(slot, impact, color, halo);
            }
            self.filled = n;
            self.next = if self.slots == 0 { 0 } else { n % self.slots };
        } else {
            if color_bits != self.built_color_bits && self.filled > 0 {
                // Cambió λ: se repinta en su sitio, sin reasignar mallas.
                let first = count.saturating_sub(self.filled);
                let mut slot = self.next;
                for impact in &sim.impacts()[first..] {
                    self.write_slot(slot, impact, color, halo);
                    slot = if self.slots == 0 {
                        0
                    } else {
                        (slot + 1) % self.slots
                    };
                }
            }
            let nuevos = (total - self.built_total)
                .min(count as u64)
                .min(self.slots as u64) as usize;
            let start = count - nuevos;
            for i in 0..nuevos {
                self.write_slot(self.next, &sim.impacts()[start + i], color, halo);
                self.next = if self.slots == 0 {
                    0
                } else {
                    (self.next + 1) % self.slots
                };
            }
            self.filled = (self.filled + nuevos).min(self.slots);
        }
        self.built_total = total;
        self.built_color_bits = color_bits;
    }

    /// Escribe un impacto (núcleo + halo) en su ranura del anillo.
    fn write_slot(
        &mut self,
        slot: usize,
        impact: &dualidad_core::Impact,
        color: Color,
        halo: Color,
    ) {
        let mesh_i = slot / QUADS_PER_MESH;
        let base = (slot % QUADS_PER_MESH) * 4;
        let x = layout::u_to_world_x(impact.u);
        let y = layout::impact_y(impact.v);
        let z = layout::SCREEN_Z - 0.15; // cara que mira a las rendijas
        let h = QUAD / 2.0;
        let vertices = &mut self.meshes[mesh_i].vertices[base..base + 4];
        vertices[0] = Vertex::new(x - h, y - h, z, 0.0, 0.0, color);
        vertices[1] = Vertex::new(x + h, y - h, z, 1.0, 0.0, color);
        vertices[2] = Vertex::new(x + h, y + h, z, 1.0, 1.0, color);
        vertices[3] = Vertex::new(x - h, y + h, z, 0.0, 1.0, color);
        if self.halo {
            // Halo suave tras el núcleo brillante (misma ranura).
            let hh = QUAD * 1.9;
            let vertices = &mut self.halo_meshes[mesh_i].vertices[base..base + 4];
            vertices[0] = Vertex::new(x - hh, y - hh, z + 0.004, 0.0, 0.0, halo);
            vertices[1] = Vertex::new(x + hh, y - hh, z + 0.004, 1.0, 0.0, halo);
            vertices[2] = Vertex::new(x + hh, y + hh, z + 0.004, 1.0, 1.0, halo);
            vertices[3] = Vertex::new(x - hh, y + hh, z + 0.004, 0.0, 1.0, halo);
        }
    }
}

/// Mallas completas del anillo, con todas las ranuras transparentes:
/// escribir una ranura es reemplazar sus 4 vértices.
fn blank_meshes(quads: usize) -> Vec<Mesh> {
    let transparent = Color::new(0.0, 0.0, 0.0, 0.0);
    let mut meshes = Vec::new();
    let mut left = quads;
    while left > 0 {
        let n = left.min(QUADS_PER_MESH);
        let mut vertices = Vec::with_capacity(n * 4);
        let mut indices = Vec::with_capacity(n * 6);
        for quad in 0..n {
            let base = (quad * 4) as u16;
            for _ in 0..4 {
                vertices.push(Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0, transparent));
            }
            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }
        meshes.push(Mesh {
            vertices,
            indices,
            texture: None,
        });
        left -= n;
    }
    meshes
}

#[cfg(test)]
mod tests {
    use super::*;
    use dualidad_core::{Simulation, SplitMix64};

    fn alfa_de(cache: &ImpactsCache, slot: usize) -> u8 {
        let mesh_i = slot / QUADS_PER_MESH;
        cache.meshes[mesh_i].vertices[(slot % QUADS_PER_MESH) * 4].color[3]
    }

    #[test]
    fn el_anillo_escribe_solo_los_impactos_nuevos() {
        let mut sim = Simulation::default();
        let mut rng = SplitMix64::new(7);
        let mut cache = ImpactsCache::default();
        sim.shoot_many(&mut rng, 3);
        cache.update(&sim, 4, true);
        assert_eq!(cache.filled, 3);
        assert_eq!(cache.next, 3);
        assert!(alfa_de(&cache, 0) > 0, "la ranura 0 tiene un impacto");
        assert_eq!(alfa_de(&cache, 3), 0, "la ranura 4 aún está vacía");
        // Más impactos: se llenan las siguientes ranuras.
        sim.shoot_many(&mut rng, 2);
        cache.update(&sim, 4, true);
        assert_eq!(cache.filled, 4, "el anillo se llena hasta su tope");
        assert_eq!(cache.next, 1, "el anillo avanza circularmente");
        // Saturado: escribe sin crecer ni reconstruir.
        let ves = cache.meshes[0].vertices.len();
        sim.shoot_many(&mut rng, 10);
        cache.update(&sim, 4, true);
        assert_eq!(cache.filled, 4);
        assert_eq!(cache.meshes[0].vertices.len(), ves, "no se reconstruye");
    }

    #[test]
    fn el_reinicio_limpia_la_pantalla() {
        let mut sim = Simulation::default();
        let mut rng = SplitMix64::new(11);
        let mut cache = ImpactsCache::default();
        sim.shoot_many(&mut rng, 2);
        cache.update(&sim, 8, false);
        assert!(alfa_de(&cache, 0) > 0);
        sim.reset();
        cache.update(&sim, 8, false);
        assert_eq!(cache.filled, 0, "sin impactos tras reiniciar");
        assert_eq!(alfa_de(&cache, 0), 0, "la ranura volvió a transparente");
    }

    #[test]
    fn el_cambio_de_lambda_repinta_sin_reasignar() {
        let mut sim = Simulation::default();
        let mut rng = SplitMix64::new(5);
        let mut cache = ImpactsCache::default();
        sim.shoot_many(&mut rng, 6);
        cache.update(&sim, 8, true);
        let vertices_antes = cache.meshes[0].vertices.len();
        let color_antes = cache.meshes[0].vertices[0].color;
        sim.params.wavelength_m = 650e-9; // rojo, lejos del verde por defecto
        cache.update(&sim, 8, true);
        assert_eq!(cache.filled, 6, "mismas ranuras");
        assert_eq!(
            cache.meshes[0].vertices.len(),
            vertices_antes,
            "no se reasignan las mallas"
        );
        assert_ne!(cache.meshes[0].vertices[0].color, color_antes);
        assert!(cache.meshes[0].vertices[0].color[3] > 0, "sigue visible");
    }

    #[test]
    fn el_cambio_de_calidad_reajusta_el_anillo() {
        let mut sim = Simulation::default();
        let mut rng = SplitMix64::new(3);
        let mut cache = ImpactsCache::default();
        sim.shoot_many(&mut rng, 30);
        cache.update(&sim, 8, true);
        assert_eq!(cache.slots, 8);
        assert_eq!(cache.filled, 8, "muestrea lo más reciente que quepa");
        cache.update(&sim, 5, false);
        assert_eq!(cache.slots, 5);
        assert_eq!(cache.filled, 5);
        assert!(cache.halo_meshes.is_empty(), "Ligera no lleva halo");
        assert!(cache.meshes.iter().all(|m| m.indices.len() <= 5_000));
    }
}
