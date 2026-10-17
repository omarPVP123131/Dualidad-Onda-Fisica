//! HUD moderno sobre la escena: marca, estado y panel de datos.
//!
//! Todo el texto pasa por el sistema de diseño (`ui.rs`, Inter), así que
//! acentos, λ y µm se dibujan bien. Los paneles son de cristal
//! redondeado con sombra, no rectángulos de debug.

use crate::color;
use crate::ui::{self, Weight};
use dualidad_core::Simulation;
use macroquad::prelude::*;

/// Datos que el HUD necesita y que no puede saber por sí mismo.
#[derive(Debug, Clone)]
pub struct HudInfo {
    pub wavelength_nm: f64,
    pub separation_um: f64,
    pub width_um: f64,
    pub observe: bool,
    pub screen_distance_m: f64,
    pub fps: i32,
    pub total: u64,
    pub rate: u32,
    pub paused: bool,
}

const CHART_BINS: usize = 96;

fn aggregate(bins: &[u32]) -> Vec<u32> {
    let mut out = vec![0u32; CHART_BINS];
    for (i, &count) in bins.iter().enumerate() {
        let idx = (i * CHART_BINS / bins.len().max(1)).min(CHART_BINS - 1);
        out[idx] += count;
    }
    out
}

/// Panel de datos: histograma observado vs curva teórica.
fn draw_chart(sim: &Simulation) {
    let (x, y, w, h) = (16.0, screen_height() - 174.0, 414.0, 158.0);
    ui::panel(x, y, w, h);
    ui::section_label(x + 14.0, y + 22.0, "HISTOGRAMA VS TEORÍA", color::ACCENT);
    ui::badge(
        x + w - 14.0,
        y + 22.0,
        &format!("{} impactos", sim.total()),
        color::ACCENT,
    );
    let (px, py, pw, ph) = (x + 14.0, y + 34.0, w - 28.0, h - 58.0);
    let hist = sim.histogram();
    if hist.total() == 0 {
        ui::text(
            "Dispara partículas y observa cómo emerge el patrón…",
            px,
            py + 26.0,
            13.0,
            color::MUTED,
            Weight::Regular,
        );
        return;
    }
    let counts = aggregate(hist.bins());
    let expected_fine = hist.expected_counts(&sim.params, sim.observe);
    let mut expected = vec![0.0f64; CHART_BINS];
    for (i, &v) in expected_fine.iter().enumerate() {
        let idx = (i * CHART_BINS / expected_fine.len().max(1)).min(CHART_BINS - 1);
        expected[idx] += v;
    }
    let scale = counts
        .iter()
        .copied()
        .map(f64::from)
        .chain(expected.iter().copied())
        .fold(1.0, f64::max);
    // Fondo de la gráfica con dos guías horizontales.
    ui::rounded_rect(px, py, pw, ph, 8.0, Color::new(0.0, 0.0, 0.0, 0.25));
    for g in [0.33, 0.66] {
        let gy = py + ph * g;
        draw_line(
            px + 4.0,
            gy,
            px + pw - 4.0,
            gy,
            1.0,
            Color::new(1.0, 1.0, 1.0, 0.06),
        );
    }
    let bw = pw / CHART_BINS as f32;
    let bar = crate::color::wave_color(sim.params.wavelength_m, 0.85);
    for (i, &c) in counts.iter().enumerate() {
        let bh = (c as f64 / scale) as f32 * ph;
        if bh > 0.5 {
            draw_rectangle(px + i as f32 * bw, py + ph - bh, bw.max(1.0), bh, bar);
        }
    }
    let mut prev: Option<(f32, f32)> = None;
    for (i, &e) in expected.iter().enumerate() {
        let cx = px + (i as f32 + 0.5) * bw;
        let cy = py + ph - (e / scale) as f32 * ph;
        if let Some((ox, oy)) = prev {
            draw_line(ox, oy, cx, cy, 1.6, WHITE);
        }
        prev = Some((cx, cy));
    }
    let dot = crate::color::wave_color(sim.params.wavelength_m, 1.0);
    draw_circle(px + 5.0, y + h - 12.0, 3.5, dot);
    ui::text(
        "impactos",
        px + 13.0,
        y + h - 8.0,
        11.5,
        color::MUTED,
        Weight::Regular,
    );
    draw_line(px + 74.0, y + h - 12.0, px + 92.0, y + h - 12.0, 1.6, WHITE);
    ui::text(
        "teoría I(x)",
        px + 97.0,
        y + h - 8.0,
        11.5,
        color::MUTED,
        Weight::Regular,
    );
}

pub fn draw_hud(sim: &Simulation, info: &HudInfo, show_chart: bool, viewport_w: f32) {
    if show_chart {
        draw_chart(sim);
    }
    // --- Marca (arriba a la izquierda del área 3D) ---
    let (x, y, w, h) = (16.0, 16.0, 438.0, 92.0);
    ui::panel(x, y, w, h);
    let accent = crate::color::wave_color(sim.params.wavelength_m, 1.0);
    ui::rounded_rect(x, y + 12.0, 4.0, h - 24.0, 2.0, accent);
    ui::text(
        "Dualidad onda–partícula",
        x + 16.0,
        y + 31.0,
        21.0,
        color::TEXT,
        Weight::Bold,
    );
    ui::text(
        "El experimento de la doble rendija, partícula a partícula",
        x + 16.0,
        y + 51.0,
        12.5,
        color::MUTED,
        Weight::Regular,
    );
    // El electrón (0.123 nm) necesita decimales: si no, se lee «λ 0 nm».
    let lambda = if info.wavelength_nm < 10.0 {
        format!("{:.3}", info.wavelength_nm)
    } else {
        format!("{:.0}", info.wavelength_nm)
    };
    let stats = format!(
        "λ {lambda} nm · d {:.0} µm · a {:.0} µm · {} impactos · {} fps",
        info.separation_um, info.width_um, info.total, info.fps
    );
    ui::text(
        &stats,
        x + 16.0,
        y + 74.0,
        12.5,
        color::TEXT,
        Weight::SemiBold,
    );

    // --- Estado del detector (pastilla, esquina del área 3D) ---
    let (label, dot) = if info.observe {
        ("Detector activado · sin interferencia", color::SENSOR_ON)
    } else {
        ("Detector apagado · hay interferencia", color::SENSOR_OFF)
    };
    let tw = ui::measure(label, 13.0, Weight::SemiBold) + 38.0;
    let px = viewport_w - tw - 16.0;
    ui::panel(px, 16.0, tw, 36.0);
    draw_circle(px + 16.0, 34.0, 5.0, dot);
    ui::text(label, px + 27.0, 38.0, 13.0, color::TEXT, Weight::SemiBold);

    if info.paused {
        let t = "⏸ En pausa";
        let tw = ui::measure(t, 20.0, Weight::Bold) + 32.0;
        let px = (viewport_w - tw) / 2.0;
        ui::panel(px, 64.0, tw, 42.0);
        ui::text(t, px + 16.0, 92.0, 20.0, color::SENSOR_ON, Weight::Bold);
    }

    // --- Ayuda breve inferior ---
    let help = "Izq: orbitar · Der: desplazar · Rueda: zoom · Alt+Rueda: lente · WASD/QE: mover · Doble clic: centrar · H: presentación";
    let tw = ui::measure(help, 12.0, Weight::Regular) + 24.0;
    let px = (viewport_w - tw) / 2.0;
    let py = screen_height() - 34.0;
    ui::rounded_rect(px, py, tw, 22.0, 11.0, Color::new(0.04, 0.05, 0.09, 0.72));
    ui::text(
        help,
        px + 12.0,
        py + 15.0,
        12.0,
        color::MUTED,
        Weight::Regular,
    );
}
