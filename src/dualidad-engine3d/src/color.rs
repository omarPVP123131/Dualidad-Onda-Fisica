//! Conversión entre el color puro del core y el color de macroquad.

use dualidad_core::Rgb;
use macroquad::prelude::Color;

pub fn to_color(rgb: Rgb, alpha: f32) -> Color {
    Color::new(rgb.r, rgb.g, rgb.b, alpha)
}

/// Color de onda para la escena: el RGB espectral puro es demasiado
/// saturado (verde lima sobre negro), así que se levanta hacia un
/// pastel luminoso sin perder el tono de λ.
pub fn wave_color(wavelength_m: f64, alpha: f32) -> Color {
    let rgb = dualidad_core::wavelength_to_rgb(wavelength_m);
    Color::new(
        0.45 + 0.55 * rgb.r,
        0.45 + 0.55 * rgb.g,
        0.45 + 0.55 * rgb.b,
        alpha,
    )
}

pub const BACKGROUND: Color = Color::new(0.027, 0.035, 0.063, 1.0);
pub const PANEL: Color = Color::new(0.055, 0.075, 0.115, 0.88);
pub const PANEL_EDGE: Color = Color::new(0.35, 0.42, 0.55, 0.65);
pub const TEXT: Color = Color::new(0.93, 0.95, 0.98, 1.0);
pub const MUTED: Color = Color::new(0.62, 0.68, 0.78, 1.0);
pub const FLOOR: Color = Color::new(0.065, 0.085, 0.125, 1.0);
pub const GRID: Color = Color::new(0.10, 0.13, 0.19, 1.0);
pub const WALL: Color = Color::new(0.13, 0.15, 0.21, 1.0);
pub const WALL_EDGE: Color = Color::new(0.36, 0.41, 0.52, 1.0);
pub const METAL: Color = Color::new(0.20, 0.23, 0.30, 1.0);
pub const SCREEN: Color = Color::new(0.012, 0.016, 0.028, 1.0);
pub const SENSOR_ON: Color = Color::new(0.98, 0.32, 0.26, 1.0);
pub const SENSOR_OFF: Color = Color::new(0.35, 0.90, 0.55, 1.0);

pub const ACCENT: Color = Color::new(0.24, 0.90, 0.62, 1.0);
