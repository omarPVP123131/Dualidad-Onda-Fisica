//! Histograma 3D frente a la pantalla, con la curva teórica en línea.
//!
//! Los 500 bins del núcleo se agregan en menos barras para que cada una
//! sea un cubo visible; la suma agregada conserva el conteo total.

use crate::color::to_color;
use crate::layout;
use dualidad_core::Simulation;
use macroquad::prelude::*;

const VISUAL_BARS: usize = 96;
const MAX_BAR_HEIGHT: f32 = 2.8;
const BARS_Z: f32 = layout::SCREEN_Z + 1.35;

/// Agrega los bins finos en `VISUAL_BARS` conteos (conserva el total).
fn aggregate(bins: &[u32]) -> Vec<u32> {
    let mut out = vec![0u32; VISUAL_BARS];
    for (i, &count) in bins.iter().enumerate() {
        let idx = (i * VISUAL_BARS / bins.len().max(1)).min(VISUAL_BARS - 1);
        out[idx] += count;
    }
    out
}

pub fn draw_histogram(sim: &Simulation) {
    let hist = sim.histogram();
    if hist.total() == 0 {
        return;
    }
    let rgb = dualidad_core::wavelength_to_rgb(sim.params.wavelength_m);
    let bar_color = to_color(rgb, 0.82);

    let counts = aggregate(hist.bins());
    let expected_fine = hist.expected_counts(&sim.params, sim.observe);
    let expected = {
        // Agregado con la misma partición que los conteos observados.
        let mut out = vec![0.0f64; VISUAL_BARS];
        for (i, &v) in expected_fine.iter().enumerate() {
            let idx = (i * VISUAL_BARS / expected_fine.len().max(1)).min(VISUAL_BARS - 1);
            out[idx] += v;
        }
        out
    };
    let scale = counts
        .iter()
        .copied()
        .map(f64::from)
        .chain(expected.iter().copied())
        .fold(1.0, f64::max);

    let bar_w = layout::SCREEN_WIDTH / VISUAL_BARS as f32;
    let mut prev: Option<Vec3> = None;
    for i in 0..VISUAL_BARS {
        let u = (i as f64 + 0.5) / VISUAL_BARS as f64;
        let x = layout::u_to_world_x(u);
        let h = (counts[i] as f64 / scale) as f32 * MAX_BAR_HEIGHT;
        if h > 0.005 {
            draw_cube(
                vec3(x, h / 2.0, BARS_Z),
                vec3(bar_w * 0.92, h, 0.5),
                None,
                bar_color,
            );
        }
        // Curva teórica: polilínea blanca sobre las barras.
        let ey = (expected[i] / scale) as f32 * MAX_BAR_HEIGHT;
        let point = vec3(x, ey, BARS_Z - 0.28);
        if let Some(p) = prev {
            draw_line_3d(p, point, WHITE);
        }
        prev = Some(point);
    }
    // Base del histograma.
    draw_cube(
        vec3(0.0, 0.02, BARS_Z),
        vec3(layout::SCREEN_WIDTH + 0.2, 0.04, 0.9),
        None,
        Color::new(0.14, 0.14, 0.18, 1.0),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn agregar_conserva_el_total() {
        let bins: Vec<u32> = (0..500).map(|i| (i % 7) as u32).collect();
        let total: u32 = bins.iter().sum();
        let agregados = super::aggregate(&bins);
        assert_eq!(agregados.len(), super::VISUAL_BARS);
        assert_eq!(agregados.iter().sum::<u32>(), total);
    }
}
