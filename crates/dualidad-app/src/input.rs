//! Traducción de teclado → acciones sobre el estado.
//!
//! Los atajos físicos son los del prototipo 2D; la cámara 3D se
//! restablece con `C` y se traslada con `W`/`A`/`S`/`D` + `Q`/`E`.
//! Ojo: `W`/`S` ya no ajustan el ancho de rendija (ese ajuste queda
//! en el deslizador del panel), porque ahora mueven la cámara.

use crate::state::AppState;
use dualidad_engine3d::Renderer;
use macroquad::prelude::*;

pub fn handle_input(state: &mut AppState, renderer: &mut Renderer, dt: f32) {
    let mut physical_change = false;

    if is_key_pressed(KeyCode::Up) {
        state.sim.params.adjust_wavelength_nm(10.0);
        physical_change = true;
    }
    if is_key_pressed(KeyCode::Down) {
        state.sim.params.adjust_wavelength_nm(-10.0);
        physical_change = true;
    }
    if is_key_pressed(KeyCode::Right) {
        state.sim.params.adjust_separation_um(5.0);
        physical_change = true;
    }
    if is_key_pressed(KeyCode::Left) {
        state.sim.params.adjust_separation_um(-5.0);
        physical_change = true;
    }
    // Cámara con WASD: traslado continuo del centro de interés
    // (la clase de movimiento que pedía el usuario). Mayús = rápido.
    {
        let fwd = (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32;
        let right = (is_key_down(KeyCode::D) as i32 - is_key_down(KeyCode::A) as i32) as f32;
        let up = (is_key_down(KeyCode::E) as i32 - is_key_down(KeyCode::Q) as i32) as f32;
        if fwd != 0.0 || right != 0.0 || up != 0.0 {
            let boost = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
            renderer.camera.move_wasd(dt, fwd, right, up, boost);
        }
    }
    if is_key_pressed(KeyCode::O) {
        state.sim.observe = !state.sim.observe;
        physical_change = true;
    }
    if is_key_pressed(KeyCode::R) {
        physical_change = true;
    }
    if is_key_pressed(KeyCode::Space) {
        state.paused = !state.paused;
    }
    if is_key_pressed(KeyCode::Equal) {
        state.rate_up();
    }
    if is_key_pressed(KeyCode::Minus) {
        state.rate_down();
    }
    if is_key_pressed(KeyCode::C) {
        renderer.camera.reset();
    }
    if is_key_pressed(KeyCode::H) {
        renderer.presentation = !renderer.presentation;
    }

    // Cambiar el experimento invalida los impactos acumulados: mezclar
    // distribuciones distintas arruinaría la comparación con la teoría.
    if physical_change {
        state.sim.reset();
        renderer.reset_visuals();
    }
}
