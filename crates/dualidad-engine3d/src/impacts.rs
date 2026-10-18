//! Impactos sobre la pantalla 3D como mallas por lotes.
//!
//! Dibujar esfera por punto (hasta 40 000) sería inviable frame a frame.
//! En su lugar cada impacto es un quad diminuto fusionado en mallas de
//! hasta `QUADS_PER_MESH` quads: unas pocas llamadas de dibujo en total.
//! La caché reconstruye solo cuando crecieron los impactos, cambiaron el
//! color (λ) o hubo reinicio.

use crate::layout;
use dualidad_core::Simulation;
use macroquad::prelude::*;

/// 800 quads = 4 800 índices: bajo el límite por geometría de macroquad.
const QUADS_PER_MESH: usize = 800;
const QUAD: f32 = 0.030;
/// Reconstruir como mucho cada N impactos nuevos (equilibrio fluidez/detalle).
const REBUILD_EVERY: usize = 25;

#[derive(Default)]
pub struct ImpactsCache {
    meshes: Vec<Mesh>,
    halo_meshes: Vec<Mesh>,
    built_count: usize,
    built_color_bits: u32,
    built_cap: usize,
    built_halo: bool,
}

impl ImpactsCache {
    pub fn draw(&mut self, sim: &Simulation, draw_cap: usize, halo_enabled: bool) {
        let rgb = dualidad_core::wavelength_to_rgb(sim.params.wavelength_m);
        let color_bits = (rgb.r.to_bits() << 16) ^ rgb.g.to_bits() ^ rgb.b.to_bits();
        let count = sim.impacts().len();
        let stale = count != self.built_count
            && (count == 0
                || count < self.built_count
                || count - self.built_count >= REBUILD_EVERY
                || color_bits != self.built_color_bits);
        let quality_changed = draw_cap != self.built_cap || halo_enabled != self.built_halo;
        if stale || quality_changed || (count == 0 && self.built_count != 0) {
            self.rebuild(sim, draw_cap, halo_enabled);
            self.built_count = count;
            self.built_color_bits = color_bits;
            self.built_cap = draw_cap;
            self.built_halo = halo_enabled;
        }
        for mesh in &self.halo_meshes {
            draw_mesh(mesh);
        }
        for mesh in &self.meshes {
            draw_mesh(mesh);
        }
    }

    fn rebuild(&mut self, sim: &Simulation, draw_cap: usize, halo_enabled: bool) {
        let color = crate::color::wave_color(sim.params.wavelength_m, 0.95);
        let halo = crate::color::wave_color(sim.params.wavelength_m, 0.16);
        self.meshes.clear();
        self.halo_meshes.clear();
        // Muestreo uniforme si hay más impactos de los que este nivel
        // dibuja: el patrón se conserva, los datos reales no se tocan.
        let all = sim.impacts();
        let step = if all.len() > draw_cap {
            all.len().div_ceil(draw_cap.max(1))
        } else {
            1
        };
        let sampled: Vec<dualidad_core::Impact> = all.iter().step_by(step).copied().collect();
        let halo_on = halo_enabled;
        for chunk in sampled.chunks(QUADS_PER_MESH) {
            let mut vertices = Vec::with_capacity(chunk.len() * 4);
            let mut indices = Vec::with_capacity(chunk.len() * 6);
            let mut halo_vertices = Vec::with_capacity(chunk.len() * 4);
            let mut halo_indices = Vec::with_capacity(chunk.len() * 6);
            for impact in chunk {
                let base = vertices.len() as u16;
                let x = layout::u_to_world_x(impact.u);
                let y = layout::impact_y(impact.v);
                let z = layout::SCREEN_Z - 0.15; // cara que mira a las rendijas
                let h = QUAD / 2.0;
                // Halo suave tras el nucleo brillante (mismo lote).
                let hh = QUAD * 1.9;
                halo_vertices.push(Vertex::new(x - hh, y - hh, z + 0.004, 0.0, 0.0, halo));
                halo_vertices.push(Vertex::new(x + hh, y - hh, z + 0.004, 1.0, 0.0, halo));
                halo_vertices.push(Vertex::new(x + hh, y + hh, z + 0.004, 1.0, 1.0, halo));
                halo_vertices.push(Vertex::new(x - hh, y + hh, z + 0.004, 0.0, 1.0, halo));
                halo_indices.extend_from_slice(&[
                    base,
                    base + 1,
                    base + 2,
                    base,
                    base + 2,
                    base + 3,
                ]);
                vertices.push(Vertex::new(x - h, y - h, z, 0.0, 0.0, color));
                vertices.push(Vertex::new(x + h, y - h, z, 1.0, 0.0, color));
                vertices.push(Vertex::new(x + h, y + h, z, 1.0, 1.0, color));
                vertices.push(Vertex::new(x - h, y + h, z, 0.0, 1.0, color));
                indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            }
            self.meshes.push(Mesh {
                vertices,
                indices,
                texture: None,
            });
            if halo_on {
                self.halo_meshes.push(Mesh {
                    vertices: halo_vertices,
                    indices: halo_indices,
                    texture: None,
                });
            }
        }
    }
}
