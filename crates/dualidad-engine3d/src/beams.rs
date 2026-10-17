//! Haces translúcidos: de la fuente a las rendijas, y de cada rendija
//! expandiéndose hacia la pantalla.
//!
//! Son volúmenes esquemáticos (frustums de vértices propios): comunican
//! el camino de la luz/materia sin afirmar trayectorias de partícula.

use crate::color;
use crate::layout;
use crate::scene;
use dualidad_core::SimulationParams;
use macroquad::prelude::*;

fn quad_indices(base: u16) -> [u16; 6] {
    [base, base + 1, base + 2, base, base + 2, base + 3]
}

/// Frustum entre dos rectángulos paralelos (caras laterales, sin tapas).
fn frustum(c0: Vec3, w0: f32, h0: f32, c1: Vec3, w1: f32, h1: f32, tint: Color) -> Mesh {
    let corners = [
        vec3(c0.x - w0 / 2.0, c0.y - h0 / 2.0, c0.z),
        vec3(c0.x + w0 / 2.0, c0.y - h0 / 2.0, c0.z),
        vec3(c0.x + w0 / 2.0, c0.y + h0 / 2.0, c0.z),
        vec3(c0.x - w0 / 2.0, c0.y + h0 / 2.0, c0.z),
        vec3(c1.x - w1 / 2.0, c1.y - h1 / 2.0, c1.z),
        vec3(c1.x + w1 / 2.0, c1.y - h1 / 2.0, c1.z),
        vec3(c1.x + w1 / 2.0, c1.y + h1 / 2.0, c1.z),
        vec3(c1.x - w1 / 2.0, c1.y + h1 / 2.0, c1.z),
    ];
    let mut vertices: Vec<Vertex> = corners
        .iter()
        .map(|p| Vertex::new(p.x, p.y, p.z, 0.0, 0.0, tint))
        .collect();
    // Duplicar para poder dar otra intensidad a las caras lejanas no es
    // necesario: 8 vértices y 4 caras laterales bastan.
    let mut indices = Vec::new();
    for side in 0..4u16 {
        let n = (side + 1) % 4;
        let base = vertices.len() as u16;
        for idx in [side, n, n + 4, side + 4] {
            let p = corners[idx as usize];
            vertices.push(Vertex::new(p.x, p.y, p.z, 0.0, 0.0, tint));
        }
        indices.extend_from_slice(&quad_indices(base));
    }
    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

/// Construye los tres haces de la escena para los parámetros actuales.
pub fn build_beams(params: &SimulationParams) -> Vec<Mesh> {
    let tint = color::wave_color(params.wavelength_m, 0.055);
    let axis = layout::OPTICAL_AXIS_Y;
    let centers = scene::slit_centers_x(params);
    let slit_w = scene::visual_slit_width(params);
    let mut meshes = vec![
        // Fuente -> doble rendija (haz colimado, fino).
        frustum(
            vec3(0.0, axis, layout::SOURCE_Z + 0.35),
            0.16,
            0.16,
            vec3(0.0, axis, layout::SLITS_Z - 0.05),
            1.15,
            0.5,
            color::wave_color(params.wavelength_m, 0.045),
        ),
    ];
    for cx in centers {
        // Cada rendija -> abanico hacia la pantalla.
        meshes.push(frustum(
            vec3(cx, axis, layout::SLITS_Z + 0.05),
            slit_w,
            0.22,
            vec3(cx * 1.7, axis, layout::SCREEN_Z - 0.12),
            3.4,
            2.3,
            tint,
        ));
    }
    meshes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn los_haces_son_geometria_valida() {
        let meshes = build_beams(&SimulationParams::default());
        assert_eq!(meshes.len(), 1 + SimulationParams::default().slit_count);
        for m in &meshes {
            assert!(m.indices.iter().all(|&i| (i as usize) < m.vertices.len()));
        }
    }
}
