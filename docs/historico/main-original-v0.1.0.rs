use macroquad::prelude::*;
use std::f64::consts::PI;

const BINS: usize = 500;
const HALF_W: f64 = 0.02; // la pantalla va de -20 mm a +20 mm
const MAX_DOTS: usize = 40_000;

struct Sim {
    lambda: f64, // longitud de onda (m)
    d: f64,      // separación entre rendijas (m)
    a: f64,      // ancho de cada rendija (m)
    l: f64,      // distancia rendijas-pantalla (m)
    observe: bool,
    bins: Vec<u32>,
    dots: Vec<Vec2>,
    total: u64,
}

impl Sim {
    fn new() -> Self {
        Self {
            lambda: 550e-9,
            d: 60e-6,
            a: 10e-6,
            l: 1.0,
            observe: false,
            bins: vec![0; BINS],
            dots: Vec::new(),
            total: 0,
        }
    }

    fn reset(&mut self) {
        self.bins.iter_mut().for_each(|b| *b = 0);
        self.dots.clear();
        self.total = 0;
    }

    /// Intensidad relativa en la pantalla (Fraunhofer):
    /// envolvente de difracción de una rendija x interferencia de dos.
    /// Con detector activo ("which-path") desaparece el término de interferencia.
    fn intensity(&self, x: f64) -> f64 {
        let k = PI * x / (self.lambda * self.l);
        let b = k * self.a;
        let sinc = if b.abs() < 1e-12 { 1.0 } else { b.sin() / b };
        let env = sinc * sinc;
        if self.observe {
            env
        } else {
            env * (k * self.d).cos().powi(2)
        }
    }

    /// Dispara UNA partícula: muestrea su posición de impacto con la
    /// distribución |psi|^2 (rechazo). Cada impacto es aleatorio, el patrón emerge.
    fn shoot(&mut self) {
        loop {
            let x = rand::gen_range(-HALF_W as f32, HALF_W as f32) as f64;
            if (rand::gen_range(0.0f32, 1.0) as f64) < self.intensity(x) {
                let u = (x + HALF_W) / (2.0 * HALF_W);
                let bin = ((u * BINS as f64) as usize).min(BINS - 1);
                self.bins[bin] += 1;
                self.total += 1;
                if self.dots.len() < MAX_DOTS {
                    self.dots.push(vec2(u as f32, rand::gen_range(0.0, 1.0)));
                }
                break;
            }
        }
    }
}

fn wavelength_color(l: f64) -> Color {
    let nm = l * 1e9;
    let (r, g, b) = match nm {
        n if n < 440.0 => ((440.0 - n) / 60.0, 0.0, 1.0),
        n if n < 490.0 => (0.0, (n - 440.0) / 50.0, 1.0),
        n if n < 510.0 => (0.0, 1.0, (510.0 - n) / 20.0),
        n if n < 580.0 => ((n - 510.0) / 70.0, 1.0, 0.0),
        n if n < 645.0 => (1.0, (645.0 - n) / 65.0, 0.0),
        _ => (1.0, 0.0, 0.0),
    };
    Color::new(r as f32, g as f32, b as f32, 1.0)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dualidad onda-partícula".to_owned(),
        window_width: 1000,
        window_height: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sim = Sim::new();
    let mut rate: u32 = 20; // partículas por frame
    let mut paused = false;

    loop {
        // ---- Controles ----
        let mut changed = false;
        if is_key_pressed(KeyCode::Up) { sim.lambda = (sim.lambda + 10e-9).min(750e-9); changed = true; }
        if is_key_pressed(KeyCode::Down) { sim.lambda = (sim.lambda - 10e-9).max(380e-9); changed = true; }
        if is_key_pressed(KeyCode::Right) { sim.d = (sim.d + 5e-6).min(200e-6); changed = true; }
        if is_key_pressed(KeyCode::Left) { sim.d = (sim.d - 5e-6).max(20e-6); changed = true; }
        if is_key_pressed(KeyCode::W) { sim.a = (sim.a + 1e-6).min(40e-6); changed = true; }
        if is_key_pressed(KeyCode::S) { sim.a = (sim.a - 1e-6).max(2e-6); changed = true; }
        if is_key_pressed(KeyCode::O) { sim.observe = !sim.observe; changed = true; }
        if is_key_pressed(KeyCode::R) { changed = true; }
        if is_key_pressed(KeyCode::Space) { paused = !paused; }
        if is_key_pressed(KeyCode::Equal) { rate = (rate * 2).min(2000); }
        if is_key_pressed(KeyCode::Minus) { rate = (rate / 2).max(1); }
        if changed { sim.reset(); }

        if !paused {
            for _ in 0..rate { sim.shoot(); }
        }

        // ---- Dibujo ----
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));
        let w = screen_width();
        let color = wavelength_color(sim.lambda);

        draw_text(
            &format!(
                "λ={:.0} nm   d={:.0} µm   a={:.0} µm   detector={}   impactos={}   ritmo={}/frame",
                sim.lambda * 1e9, sim.d * 1e6, sim.a * 1e6,
                if sim.observe { "ON" } else { "OFF" }, sim.total, rate
            ),
            20.0, 28.0, 22.0, WHITE,
        );
        draw_text(
            "↑↓ λ   ←→ d   W/S ancho   O detector   +/- ritmo   Espacio pausa   R reiniciar",
            20.0, 50.0, 18.0, GRAY,
        );

        // Pantalla de detección: cada punto es una partícula
        let (sx, sy, sw, sh) = (40.0, 70.0, w - 80.0, 200.0);
        draw_rectangle(sx, sy, sw, sh, BLACK);
        for p in &sim.dots {
            draw_rectangle(sx + p.x * sw, sy + p.y * sh, 2.0, 2.0, color);
        }
        draw_rectangle_lines(sx, sy, sw, sh, 1.0, DARKGRAY);

        // Histograma vs. curva teórica
        let (hx, hy, hw, hh) = (40.0, 300.0, w - 80.0, 360.0);
        let sum_i: f64 = (0..BINS)
            .map(|i| sim.intensity(-HALF_W + (i as f64 + 0.5) / BINS as f64 * 2.0 * HALF_W))
            .sum();
        let expected = |i: usize| -> f64 {
            let x = -HALF_W + (i as f64 + 0.5) / BINS as f64 * 2.0 * HALF_W;
            sim.total as f64 * sim.intensity(x) / sum_i
        };
        let max_v = (0..BINS)
            .map(|i| (sim.bins[i] as f64).max(expected(i)))
            .fold(1.0, f64::max);
        let bw = hw / BINS as f32;
        for i in 0..BINS {
            let bar = (sim.bins[i] as f64 / max_v) as f32 * hh;
            draw_rectangle(hx + i as f32 * bw, hy + hh - bar, bw, bar, Color::new(color.r, color.g, color.b, 0.6));
        }
        for i in 1..BINS {
            let y0 = hy + hh - (expected(i - 1) / max_v) as f32 * hh;
            let y1 = hy + hh - (expected(i) / max_v) as f32 * hh;
            draw_line(hx + (i - 1) as f32 * bw, y0, hx + i as f32 * bw, y1, 2.0, WHITE);
        }
        draw_rectangle_lines(hx, hy, hw, hh, 1.0, DARKGRAY);
        draw_text("Histograma de impactos (barras) vs. teoría (línea)", hx, hy - 6.0, 18.0, GRAY);

        next_frame().await
    }
}