//! Superficie 3D de la distribución de probabilidad `I(x)`.
//!
//! La malla va de las rendijas (z = 0) a la pantalla (z = 6). A lo largo
//! del eje transversal (x) sigue `I(x)`; al avanzar en z crece hasta el
//! patrón final que «llega» a la pantalla. Es una visualización de la
//! distribución, no una solución de la ecuación de Schrödinger.
//!
//! La superficie se emite en tiras: macroquad recorta cada geometría a
//! 5 000 índices, y una sola malla los superaría con creces.

use crate::color::to_color;
use crate::layout;
use dualidad_core::{physics, Simulation, SimulationParams};
use macroquad::prelude::*;

const NX: usize = 180;
const NZ: usize = 56;
/// Filas por tira: 180 × 4 × 6 = 4 320 índices, bajo el límite de 5 000.
const ROWS_PER_MESH: usize = 4;
const MAX_HEIGHT: f32 = 0.95;
/// La superficie flota a la altura del haz, no nace del suelo.
const BASE_Y: f32 = 1.02;

/// Altura de la superficie en función de la posición normalizada `u` y
/// del avance `t ∈ [0, 1]` entre rendijas y pantalla.
fn height(params: &SimulationParams, observe: bool, u: f64, t: f32) -> f32 {
    let x = params.to_position(u);
    let i = physics::intensity(params, x, observe) as f32;
    // Nace casi plana en las rendijas y crece hasta el patrón en la pantalla.
    i * MAX_HEIGHT * (0.12 + 0.88 * t * t)
}

/// Construye las mallas de la superficie para el estado actual (en tiras).
/// `resolution` = (columnas, filas) según el nivel de calidad: en modo
/// ligero la malla es más gruesa y el render por software respira.
pub fn build_wave_meshes(sim: &Simulation, resolution: (usize, usize)) -> Vec<Mesh> {
    let (nx, nz) = resolution;
    let color = to_color(
        dualidad_core::wavelength_to_rgb(sim.params.wavelength_m),
        1.0,
    );
    let mut meshes = Vec::new();
    let mut j0 = 0;
    while j0 < nz {
        let j1 = (j0 + ROWS_PER_MESH).min(nz);
        meshes.push(build_strip(sim, color, j0, j1, nx, nz));
        j0 = j1;
    }
    meshes
}

fn build_strip(sim: &Simulation, color: Color, j0: usize, j1: usize, nx: usize, nz: usize) -> Mesh {
    let mut vertices = Vec::with_capacity((nx + 1) * (j1 - j0 + 1));
    let mut indices = Vec::with_capacity(nx * (j1 - j0) * 6);

    for j in j0..=j1 {
        let t = j as f32 / nz as f32;
        let z = layout::SLITS_Z + t * (layout::SCREEN_Z - layout::SLITS_Z);
        for i in 0..=nx {
            let u = i as f64 / nx as f64;
            let y = BASE_Y + height(&sim.params, sim.observe, u, t);
            // El color se aclara con la altura (más probable = más intenso).
            let f = ((y - BASE_Y) / MAX_HEIGHT).clamp(0.0, 1.0);
            let c = Color::new(
                color.r * (0.16 + 0.62 * f),
                color.g * (0.16 + 0.62 * f),
                color.b * (0.16 + 0.62 * f),
                0.62,
            );
            vertices.push(Vertex::new(layout::u_to_world_x(u), y, z, u as f32, t, c));
        }
    }
    let row = (nx + 1) as u16;
    for j in 0..(j1 - j0) as u16 {
        for i in 0..nx as u16 {
            let a = j * row + i;
            let b = a + 1;
            let c = a + row;
            let d = c + 1;
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

/// Contornos tenues sobre la superficie: filas (z fijo) y columnas
/// (x fijo) que hacen legible la forma sin anadir geometria pesada.
pub fn draw_wave_contours(sim: &Simulation) {
    let line = Color::new(1.0, 1.0, 1.0, 0.16);
    let at = |u: f64, t: f32| {
        vec3(
            layout::u_to_world_x(u),
            BASE_Y + height(&sim.params, sim.observe, u, t) + 0.006,
            layout::SLITS_Z + t * (layout::SCREEN_Z - layout::SLITS_Z),
        )
    };
    for j in (0..=NZ).step_by(8) {
        let t = j as f32 / NZ as f32;
        let mut prev = at(0.0, t);
        for i in (2..=NX).step_by(2) {
            let cur = at(i as f64 / NX as f64, t);
            draw_line_3d(prev, cur, line);
            prev = cur;
        }
    }
    for i in (0..=NX).step_by(20) {
        let u = i as f64 / NX as f64;
        let mut prev = at(u, 0.0);
        for j in (2..=NZ).step_by(2) {
            let cur = at(u, j as f32 / NZ as f32);
            draw_line_3d(prev, cur, line);
            prev = cur;
        }
    }
}

/// Frentes de onda animados: perfiles brillantes que viajan de las
/// rendijas a la pantalla sobre la superficie, en bucle.
pub fn draw_wavefronts(sim: &Simulation, time: f32) {
    let bright = crate::color::wave_color(sim.params.wavelength_m, 0.85);
    for k in 0..3 {
        let t = (time * 0.28 + k as f32 / 3.0).fract();
        let z = layout::SLITS_Z + t * (layout::SCREEN_Z - layout::SLITS_Z);
        let fade = 1.0 - (t - 0.5).abs() * 1.4; // más visible a media escena
        let c = Color::new(bright.r, bright.g, bright.b, 0.25 + 0.55 * fade.max(0.0));
        let mut prev: Option<Vec3> = None;
        for i in (0..=NX).step_by(3) {
            let u = i as f64 / NX as f64;
            let cur = vec3(
                layout::u_to_world_x(u),
                BASE_Y + height(&sim.params, sim.observe, u, t) + 0.012,
                z,
            );
            if let Some(p0) = prev {
                draw_line_3d(p0, cur, c);
            }
            prev = Some(cur);
        }
    }
    // Frentes circulares entre la fuente y las rendijas.
    let axis = vec3(0.0, layout::OPTICAL_AXIS_Y, 0.0);
    for k in 0..4 {
        let f = (time * 0.45 + k as f32 / 4.0).fract();
        let z = layout::SOURCE_Z + 0.45 + f * (layout::SLITS_Z - layout::SOURCE_Z - 0.8);
        let r = 0.16 + f * 0.30;
        let mut prev: Option<Vec3> = None;
        for s in 0..=28 {
            let a = s as f32 / 28.0 * std::f32::consts::TAU;
            let cur = vec3(axis.x + a.cos() * r, axis.y + a.sin() * r, z);
            if let Some(p0) = prev {
                draw_line_3d(p0, cur, Color::new(bright.r, bright.g, bright.b, 0.35));
            }
            prev = Some(cur);
        }
    }
}

/// Perfil brillante donde la superficie toca la pantalla (el “resultado”).
pub fn draw_profile_glow(sim: &Simulation) {
    let bright = crate::color::wave_color(sim.params.wavelength_m, 1.0);
    for (dy, alpha) in [(0.0, 0.95), (-0.028, 0.35), (0.028, 0.35)] {
        let mut prev: Option<Vec3> = None;
        for i in (0..=NX).step_by(2) {
            let u = i as f64 / NX as f64;
            let cur = vec3(
                layout::u_to_world_x(u),
                BASE_Y + height(&sim.params, sim.observe, u, 1.0) + dy,
                layout::SCREEN_Z - 0.20,
            );
            if let Some(p0) = prev {
                draw_line_3d(p0, cur, Color::new(bright.r, bright.g, bright.b, alpha));
            }
            prev = Some(cur);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_superficie_crece_hacia_la_pantalla() {
        let p = SimulationParams::default();
        let salida = height(&p, false, 0.5, 0.0);
        let pantalla = height(&p, false, 0.5, 1.0);
        assert!(pantalla > salida);
        assert!(
            (pantalla - MAX_HEIGHT).abs() < 1e-4,
            "el centro llega al máximo"
        );
    }

    #[test]
    fn dimensiones_de_las_mallas() {
        let sim = Simulation::default();
        let meshes = build_wave_meshes(&sim, (NX, NZ));
        assert!(meshes.len() > 1, "debe venir en tiras");
        for mesh in &meshes {
            // Ninguna geometría supera el límite de macroquad (5000 índices).
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
        let total_indices: usize = meshes.iter().map(|m| m.indices.len()).sum();
        assert_eq!(total_indices, NX * NZ * 6);
    }
}
