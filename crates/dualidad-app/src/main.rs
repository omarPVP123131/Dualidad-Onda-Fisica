//! Aplicación de escritorio: dualidad onda-partícula en 3D.
//!
//! Este binario solo pega las piezas:
//! `dualidad-core` (física y simulación) + `dualidad-engine3d` (escena).

mod input;
mod state;

use dualidad_engine3d::{HudInfo, Renderer};
use macroquad::prelude::*;
use state::AppState;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dualidad onda-partícula — 3D".to_owned(),
        window_width: 1440,
        window_height: 900,
        ..Default::default()
    }
}

/// Semilla a partir del reloj: cada sesión dispara una secuencia distinta.
fn seed_from_clock() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64 ^ 0xD1B5_4A32_D192_ED03)
        .unwrap_or(0x853c_49e6_748f_ea9b)
}

/// Dispara `n` partículas y alimenta las partículas en vuelo con los
/// impactos recién generados.
///
/// Se toman los `hechos` **últimos** del almacén: el núcleo conserva los
/// impactos recientes en una ventana deslizante, así que el conteo que
/// devuelve `shoot_many` es la vía fiable. El viejo recorte por índices
/// `before..after` dejaba de notificar al llenarse el almacén (ADR-0005).
fn disparar(state: &mut AppState, renderer: &mut Renderer, n: u32) {
    let hechos = state.sim.shoot_many(&mut state.rng, n) as usize;
    if hechos == 0 {
        return;
    }
    let impactos = state.sim.impacts();
    let desde = impactos.len().saturating_sub(hechos);
    renderer.notify_impacts(&state.sim, &impactos[desde..]);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::new(seed_from_clock());
    let mut renderer = Renderer::new();

    loop {
        let dt = get_frame_time();

        input::handle_input(&mut state, &mut renderer, dt);
        renderer.update_camera(dt);

        if !state.paused {
            let rate = state.rate;
            disparar(&mut state, &mut renderer, rate);
        }

        clear_background(Color::new(0.03, 0.03, 0.05, 1.0));
        let hud = HudInfo {
            wavelength_nm: state.sim.params.wavelength_nm(),
            separation_um: state.sim.params.separation_um(),
            width_um: state.sim.params.width_um(),
            observe: state.sim.observe,
            screen_distance_m: state.sim.params.screen_distance_m,
            fps: get_fps(),
            total: state.sim.total(),
            rate: state.rate,
            paused: state.paused,
        };
        let outcome = renderer.render(&mut state.sim, &hud, dt, &mut state.rate, &mut state.paused);
        if outcome.shoot > 0 {
            disparar(&mut state, &mut renderer, outcome.shoot);
        }

        next_frame().await
    }
}
