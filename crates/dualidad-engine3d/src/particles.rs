//! Partículas en vuelo: recurso visual para seguir cada disparo.
//!
//! La trayectoria dibujada es **esquemática** (fuente → una rendija →
//! impacto en pantalla). No afirma un camino medido: cuando no hay
//! detector, elegir una rendija para el dibujo es solo un artificio
//! gráfico; el resultado físico sigue siendo la acumulación estadística.
//! La física real del proyecto vive en `dualidad-core`.

use crate::layout;
use crate::scene;
use dualidad_core::{Impact, SimulationParams};
use macroquad::prelude::*;

const MAX_ACTIVE: usize = 240;
const MAX_SPAWN_PER_FRAME: usize = 8;
const DURATION: f32 = 0.55;

#[derive(Debug, Clone, Copy)]
struct Flying {
    via_x: f32,
    target: Vec3,
    age: f32,
    prev: Option<Vec3>,
}

#[derive(Default)]
pub struct FlightParticles {
    active: Vec<Flying>,
}

impl FlightParticles {
    /// Registra impactos nuevos como partículas en vuelo (con tope).
    pub fn spawn(&mut self, params: &SimulationParams, impacts: &[Impact], cap: usize) {
        if impacts.is_empty() {
            return;
        }
        let step = (impacts.len() / MAX_SPAWN_PER_FRAME).max(1);
        let centers = scene::slit_centers_x(params);
        let cap = cap.min(MAX_ACTIVE);
        for impact in impacts.iter().step_by(step).take(MAX_SPAWN_PER_FRAME) {
            // Con el buffer lleno se recicla la más antigua: a ritmos
            // altos el flujo se mantiene en vez de congelarse.
            if self.active.len() >= cap && !self.active.is_empty() {
                self.active.remove(0);
            }
            let x = layout::u_to_world_x(impact.u);
            // Rendija «de paso» puramente gráfica: la más cercana al destino.
            let via_x = centers
                .iter()
                .copied()
                .min_by(|a, b| (x - a).abs().total_cmp(&(x - b).abs()))
                .unwrap_or(0.0);
            self.active.push(Flying {
                via_x,
                prev: None,
                target: vec3(x, layout::impact_y(impact.v), layout::SCREEN_Z - 0.18),
                age: 0.0,
            });
        }
    }

    pub fn update_and_draw(
        &mut self,
        dt: f32,
        wavelength_m: f64,
        tier: crate::quality::QualityTier,
    ) {
        let color = crate::color::wave_color(wavelength_m, 1.0);
        let source = vec3(0.0, layout::OPTICAL_AXIS_Y, layout::SOURCE_Z);
        self.active.retain_mut(|p| {
            p.age += dt;
            let t = p.age / DURATION;
            if t >= 1.0 {
                return false;
            }
            // Dos tramos: fuente → rendija (0–45%), rendija → pantalla (45–100%).
            let via = vec3(p.via_x, layout::OPTICAL_AXIS_Y, layout::SLITS_Z);
            let pos = if t < 0.45 {
                source.lerp(via, t / 0.45)
            } else {
                via.lerp(p.target, (t - 0.45) / 0.55)
            };
            // Aparece y se desvanece con suavidad: sin esto la partícula
            // «explotaba» en la fuente y desaparecía de golpe al llegar.
            let fade_in = (t / 0.08).min(1.0);
            let fade_out = ((1.0 - t) / 0.15).min(1.0);
            let fade = fade_in.min(fade_out).clamp(0.0, 1.0);
            let tint = Color::new(color.r, color.g, color.b, color.a * fade);
            if tier != crate::quality::QualityTier::Ligera {
                if let Some(p0) = p.prev {
                    draw_line_3d(p0, pos, Color::new(color.r, color.g, color.b, 0.45 * fade));
                }
            }
            p.prev = Some(pos);
            // Esferas solo en Alta: en modo ligero un cubo de 12
            // triangulos cuesta una fraccion y se lee igual de bien.
            if tier == crate::quality::QualityTier::Alta {
                draw_sphere(pos, 0.042 * (0.35 + 0.65 * fade), None, tint);
            } else {
                let s = 0.075 * (0.35 + 0.65 * fade);
                draw_cube(pos, vec3(s, s, s), None, tint);
            }
            true
        });
    }

    pub fn clear(&mut self) {
        self.active.clear();
    }

    pub fn active_count(&self) -> usize {
        self.active.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dualidad_core::SimulationParams;

    fn impactos(n: usize) -> Vec<Impact> {
        (0..n)
            .map(|i| Impact {
                x_m: 0.0,
                u: (i as f64 % 97.0) / 97.0,
                v: (i as f64 % 53.0) / 53.0,
            })
            .collect()
    }

    #[test]
    fn el_destino_cae_sobre_la_pantalla() {
        let params = SimulationParams::default();
        let mut p = FlightParticles::default();
        let mut extremos = impactos(2);
        extremos[0].v = 0.0;
        extremos[1].v = 1.0;
        p.spawn(&params, &extremos, 240);
        assert_eq!(p.active.len(), 2);
        let bottom = layout::OPTICAL_AXIS_Y - layout::SCREEN_HEIGHT / 2.0;
        let top = layout::OPTICAL_AXIS_Y + layout::SCREEN_HEIGHT / 2.0;
        for f in &p.active {
            assert!(
                f.target.y > bottom && f.target.y < top,
                "destino fuera de la pantalla: y={}",
                f.target.y
            );
        }
    }

    #[test]
    fn con_el_buffer_lleno_se_recicla_la_mas_antigua() {
        let params = SimulationParams::default();
        let mut p = FlightParticles::default();
        p.spawn(&params, &impactos(8), 4);
        let lleno = p.active_count();
        p.spawn(&params, &impactos(8), 4);
        assert_eq!(p.active_count(), lleno, "el tope se mantiene reciclando");
    }
}
