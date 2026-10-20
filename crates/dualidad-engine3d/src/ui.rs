//! Sistema de diseño de la interfaz 2D.
//!
//! Tipografía Inter incrustada (SIL OFL, ver `assets/fonts/OFL.txt`),
//! paneles redondeados tipo cristal y la paleta compartida por HUD,
//! panel de control y anotaciones. Sin esto la UI se veía a debug:
//! fuente del sistema sin acentos ni λ, rectángulos duros y texto plano.

use crate::color;
use macroquad::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    Regular,
    SemiBold,
    Bold,
}

struct Fonts {
    regular: Font,
    semibold: Font,
    bold: Font,
}

thread_local! {
    // El inicializador ya es `const`; clippy 1.99 lo señala igualmente.
    #[allow(clippy::missing_const_for_thread_local)]
    static FONTS: RefCell<Option<Fonts>> = const { RefCell::new(None) };
}

fn with_fonts<R>(f: impl FnOnce(&Fonts) -> R) -> R {
    FONTS.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            let load = |bytes: &[u8]| {
                load_ttf_font_from_bytes(bytes).expect("tipografía Inter incrustada válida")
            };
            *slot = Some(Fonts {
                regular: load(include_bytes!("../assets/fonts/Inter-Regular.ttf")),
                semibold: load(include_bytes!("../assets/fonts/Inter-SemiBold.ttf")),
                bold: load(include_bytes!("../assets/fonts/Inter-Bold.ttf")),
            });
        }
        f(slot.as_ref().expect("fuentes inicializadas"))
    })
}

fn font(weight: Weight) -> Font {
    with_fonts(|fonts| match weight {
        Weight::Regular => fonts.regular.clone(),
        Weight::SemiBold => fonts.semibold.clone(),
        Weight::Bold => fonts.bold.clone(),
    })
}

/// Texto con la tipografía del sistema de diseño.
pub fn text(value: &str, x: f32, y: f32, size: f32, tint: Color, weight: Weight) {
    let font = font(weight);
    draw_text_ex(
        value,
        x,
        y,
        TextParams {
            font: Some(&font),
            font_size: size as u16,
            color: tint,
            ..Default::default()
        },
    );
}

pub fn measure(value: &str, size: f32, weight: Weight) -> f32 {
    let font = font(weight);
    measure_text(value, Some(&font), size as u16, 1.0).width
}

/// Rectángulo de esquinas redondeadas (relleno).
pub fn rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, tint: Color) {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    draw_rectangle(x + r, y, w - 2.0 * r, h, tint);
    draw_rectangle(x, y + r, w, h - 2.0 * r, tint);
    for (cx, cy) in [
        (x + r, y + r),
        (x + w - r, y + r),
        (x + r, y + h - r),
        (x + w - r, y + h - r),
    ] {
        draw_circle(cx, cy, r, tint);
    }
}

/// Panel flotante: sombra suave + cristal oscuro + borde sutil.
pub fn panel(x: f32, y: f32, w: f32, h: f32) {
    rounded_rect(
        x + 3.0,
        y + 5.0,
        w,
        h,
        14.0,
        Color::new(0.0, 0.0, 0.0, 0.35),
    );
    rounded_rect(x, y, w, h, 14.0, color::PANEL);
    rounded_rect(x, y, w, 1.2, 0.6, Color::new(1.0, 1.0, 1.0, 0.07));
    // Borde aproximado con un marco de esquinas redondeadas detrás.
    rounded_rect(
        x - 1.0,
        y - 1.0,
        w + 2.0,
        h + 2.0,
        15.0,
        Color::new(0.55, 0.65, 0.85, 0.10),
    );
    rounded_rect(x, y, w, h, 14.0, color::PANEL);
    rounded_rect(x, y, w, 1.2, 0.6, Color::new(1.0, 1.0, 1.0, 0.07));
}

/// Etiqueta de sección: barra de acento + mayúsculas con tracking visual.
pub fn section_label(x: f32, y: f32, label: &str, accent: Color) {
    rounded_rect(x, y - 10.0, 3.0, 12.0, 1.5, accent);
    text(label, x + 9.0, y, 12.0, color::MUTED, Weight::SemiBold);
}

/// Pastilla de valor (badge) alineada a la derecha de `right_x`.
pub fn badge(right_x: f32, y: f32, value: &str, accent: Color) {
    let tw = measure(value, 12.0, Weight::SemiBold);
    let w = tw + 16.0;
    rounded_rect(
        right_x - w,
        y - 11.0,
        w,
        18.0,
        9.0,
        Color::new(accent.r, accent.g, accent.b, 0.16),
    );
    text(
        value,
        right_x - w + 8.0,
        y + 2.0,
        12.0,
        color::TEXT,
        Weight::SemiBold,
    );
}

/// Interruptor (switch) moderno. Devuelve el rectángulo para hit-test.
pub fn switch(x: f32, y: f32, on: bool, accent: Color) -> (f32, f32, f32, f32) {
    let (w, h) = (36.0, 20.0);
    let bg = if on {
        accent
    } else {
        Color::new(0.25, 0.29, 0.38, 1.0)
    };
    rounded_rect(x, y, w, h, h / 2.0, bg);
    let knob_x = if on { x + w - 10.0 } else { x + 10.0 };
    draw_circle(knob_x, y + h / 2.0, 7.0, Color::new(0.06, 0.07, 0.10, 0.35));
    draw_circle(knob_x, y + h / 2.0 - 0.7, 6.4, WHITE);
    (x, y, w, h)
}
