//! Panel de control moderno: dock lateral de cristal con sliders,
//! presets en tarjetas, vistas y capas.
//!
//! Widgets propios sobre el sistema de diseño (`ui.rs`): esquinas
//! redondeadas, interruptores, badges de valor y tarjetas con el color
//! de cada preset. La lógica de interacción no cambia: lee y escribe el
//! mismo estado que el teclado, y ambos se sincronizan solos.

use crate::camera::{OrbitCamera, ViewPreset};
use crate::color;
use crate::quality::{QualityMode, QualityTier};
use crate::ui::{self, Weight};
use dualidad_core::presets::catalog;
use dualidad_core::Simulation;
use macroquad::prelude::*;

/// Capas visuales activables de la escena.
#[derive(Debug, Clone, Copy)]
pub struct Layers {
    pub wave: bool,
    pub beams: bool,
    pub particles: bool,
    pub annotations: bool,
    pub chart: bool,
}

impl Default for Layers {
    fn default() -> Self {
        Self {
            wave: true,
            beams: true,
            particles: true,
            annotations: true,
            chart: true,
        }
    }
}

/// Lo que el panel pide a la app tras un frame.
#[derive(Debug, Default, Clone, Copy)]
pub struct ControlOutcome {
    pub reset_visuals: bool,
    pub shoot: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SliderId {
    Lambda,
    Separation,
    Width,
    Distance,
    Rate,
}

#[derive(Default)]
pub struct ControlPanel {
    active: Option<SliderId>,
    /// Desplazamiento del contenido del panel (px) y su máximo.
    scroll: f32,
    max_scroll: f32,
    /// Offset de agarre dentro del pulgar de la barra, si se arrastra.
    scroll_drag: Option<f32>,
}

/// ¿Cae completo el widget dentro de la zona visible del panel?
/// (macroquad no ofrece recorte público, así que el contenido con
/// scroll no se dibuja —ni responde— fuera de su zona.)
fn in_view(top: f32, height: f32, view: (f32, f32)) -> bool {
    top >= view.0 && top + height <= view.1
}

/// Alto máximo del contenido desplazable para una vista dada.
fn scroll_max(content_h: f32, view_h: f32) -> f32 {
    (content_h - view_h).max(0.0)
}

/// Geometría de la barra: (alto del pulgar, desplazamiento desde arriba).
fn thumb_geometry(view_h: f32, content_h: f32, scroll: f32) -> (f32, f32) {
    if content_h <= view_h {
        return (view_h, 0.0);
    }
    let thumb_h = (view_h * view_h / content_h).clamp(26.0, view_h);
    let frac = (scroll / (content_h - view_h)).clamp(0.0, 1.0);
    (thumb_h, frac * (view_h - thumb_h))
}

pub const SIDEBAR_W: f32 = 326.0;

fn inside(x: f32, y: f32, w: f32, h: f32) -> bool {
    let (mx, my) = mouse_position();
    mx >= x && mx <= x + w && my >= y && my <= y + h
}

impl ControlPanel {
    pub fn panel_rect() -> (f32, f32, f32, f32) {
        (
            screen_width() - SIDEBAR_W - 12.0,
            12.0,
            SIDEBAR_W,
            screen_height() - 24.0,
        )
    }

    pub fn pointer_over(&self) -> bool {
        let (x, y, w, h) = Self::panel_rect();
        inside(x, y, w, h)
    }

    fn button(x: f32, y: f32, w: f32, h: f32, label: &str, active: bool, view: (f32, f32)) -> bool {
        if !in_view(y, h, view) {
            return false;
        }
        let hover = inside(x, y, w, h);
        let (bg, fg) = if active {
            (color::ACCENT, Color::new(0.04, 0.10, 0.07, 1.0))
        } else if hover {
            (Color::new(0.16, 0.20, 0.29, 0.97), color::TEXT)
        } else {
            (Color::new(0.10, 0.13, 0.20, 0.97), color::TEXT)
        };
        ui::rounded_rect(x, y, w, h, 9.0, bg);
        let tw = ui::measure(label, 13.0, Weight::SemiBold);
        ui::text(
            label,
            x + (w - tw) / 2.0,
            y + h / 2.0 + 4.5,
            13.0,
            fg,
            Weight::SemiBold,
        );
        hover && is_mouse_button_pressed(MouseButton::Left)
    }

    fn preset_card(
        x: f32,
        y: f32,
        w: f32,
        name: &str,
        detail: &str,
        dot: Color,
        view: (f32, f32),
    ) -> bool {
        if !in_view(y, 38.0, view) {
            return false;
        }
        let hover = inside(x, y, w, 38.0);
        let bg = if hover {
            Color::new(0.16, 0.20, 0.29, 0.97)
        } else {
            Color::new(0.10, 0.13, 0.20, 0.97)
        };
        ui::rounded_rect(x, y, w, 38.0, 10.0, bg);
        draw_circle(x + 14.0, y + 19.0, 5.0, dot);
        ui::text(
            name,
            x + 26.0,
            y + 17.0,
            13.0,
            color::TEXT,
            Weight::SemiBold,
        );
        ui::text(
            detail,
            x + 26.0,
            y + 31.0,
            10.5,
            color::MUTED,
            Weight::Regular,
        );
        hover && is_mouse_button_pressed(MouseButton::Left)
    }

    #[allow(clippy::too_many_arguments)]
    fn slider(
        &mut self,
        id: SliderId,
        x: f32,
        y: f32,
        w: f32,
        label: &str,
        value_text: &str,
        fraction: f32,
        accent: Color,
        view: (f32, f32),
    ) -> Option<f32> {
        if !in_view(y, 22.0, view) {
            return None;
        }
        ui::text(label, x, y + 3.0, 13.0, color::MUTED, Weight::Regular);
        ui::badge(x + w, y + 3.0, value_text, accent);
        let (tx, ty) = (x, y + 12.0);
        ui::rounded_rect(tx, ty, w, 6.0, 3.0, Color::new(0.22, 0.27, 0.37, 1.0));
        let fx = tx + fraction.clamp(0.0, 1.0) * w;
        if fx > tx {
            ui::rounded_rect(tx, ty, fx - tx, 6.0, 3.0, accent);
        }
        draw_circle(fx, ty + 3.0, 8.0, Color::new(0.0, 0.0, 0.0, 0.35));
        draw_circle(fx, ty + 2.2, 7.0, WHITE);

        let over = inside(tx - 5.0, ty - 8.0, w + 10.0, 22.0);
        if over && is_mouse_button_pressed(MouseButton::Left) {
            self.active = Some(id);
        }
        if self.active == Some(id) {
            if is_mouse_button_down(MouseButton::Left) {
                let (mx, _) = mouse_position();
                return Some(((mx - tx) / w).clamp(0.0, 1.0));
            }
            self.active = None;
        }
        None
    }

    /// Lee la rueda sobre el panel y el arrastre del pulgar de la
    /// barra. Usa el `max_scroll` del frame anterior (se recalca al
    /// final de cada `interact`): un frame de latencia, inapreciable.
    fn update_scroll(&mut self, panel_x: f32, panel_w: f32, view: (f32, f32)) {
        let (ctop, cbot) = view;
        let view_h = (cbot - ctop).max(1.0);
        let (thumb_h, thumb_dy) = thumb_geometry(view_h, view_h + self.max_scroll, self.scroll);
        let thumb_y = ctop + thumb_dy;
        let rail_x = panel_x + panel_w - 8.0;
        let (mx, my) = mouse_position();
        if self.scroll_drag.is_none()
            && self.max_scroll > 0.0
            && is_mouse_button_pressed(MouseButton::Left)
            && mx >= rail_x - 3.0
            && mx <= rail_x + 7.0
            && my >= ctop - 2.0
            && my <= cbot + 2.0
        {
            if my >= thumb_y && my <= thumb_y + thumb_h {
                self.scroll_drag = Some(my - thumb_y);
            } else {
                // Clic en el riel: salto de casi una página hacia ahí.
                let dir = if my < thumb_y { -1.0 } else { 1.0 };
                self.scroll = (self.scroll + dir * view_h * 0.85).clamp(0.0, self.max_scroll);
                self.scroll_drag = Some(thumb_h / 2.0);
            }
        }
        if let Some(grab) = self.scroll_drag {
            if is_mouse_button_down(MouseButton::Left) {
                let range = (view_h - thumb_h).max(1.0);
                let frac = ((my - grab - ctop) / range).clamp(0.0, 1.0);
                self.scroll = frac * self.max_scroll;
            } else {
                self.scroll_drag = None;
            }
        } else if self.pointer_over() && self.active.is_none() {
            let wheel = mouse_wheel().1;
            if wheel != 0.0 {
                self.scroll = (self.scroll - wheel * 42.0).clamp(0.0, self.max_scroll);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn interact(
        &mut self,
        sim: &mut Simulation,
        rate: &mut u32,
        paused: &mut bool,
        layers: &mut Layers,
        camera: &mut OrbitCamera,
        cinematic: &mut bool,
        presentation: &mut bool,
        quality_mode: &mut QualityMode,
        effective_quality: QualityTier,
    ) -> ControlOutcome {
        let mut out = ControlOutcome::default();
        let (x, y, w, h) = Self::panel_rect();
        ui::panel(x, y, w, h);
        let ix = x + 16.0;
        let iw = w - 32.0;
        let accent = crate::color::wave_color(sim.params.wavelength_m, 1.0);

        // Cabecera fija; el contenido vive bajo ella en una zona con
        // scroll (rueda o barra), para que todas las opciones se
        // alcancen también en ventanas bajas.
        ui::text(
            "Control del experimento",
            ix,
            y + 30.0,
            18.0,
            color::TEXT,
            Weight::Bold,
        );
        ui::text(
            "Ajusta, dispara y mira qué cambia",
            ix,
            y + 50.0,
            12.0,
            color::MUTED,
            Weight::Regular,
        );
        let view = (y + 66.0, y + h - 12.0);
        let view_h = (view.1 - view.0).max(1.0);
        self.update_scroll(x, w, view);
        let mut cy = view.0 - self.scroll;

        // Estado del detector con interruptor.
        if in_view(cy, 30.0, view) {
            ui::text(
                "Detector which-path",
                ix,
                cy + 14.0,
                13.5,
                color::TEXT,
                Weight::SemiBold,
            );
            let (sx, sy, sw, sh) = ui::switch(ix + iw - 36.0, cy, sim.observe, color::SENSOR_ON);
            if inside(sx, sy, sw, sh) && is_mouse_button_pressed(MouseButton::Left) {
                sim.observe = !sim.observe;
                sim.reset();
                out.reset_visuals = true;
            }
        }
        cy += 32.0;

        if in_view(cy, 14.0, view) {
            ui::section_label(ix, cy, "EXPERIMENTO", accent);
        }
        cy += 8.0;
        for (i, preset) in catalog().iter().enumerate() {
            let col = i % 2;
            let row = i / 2;
            let bx = ix + col as f32 * (iw / 2.0 + 5.0);
            let by = cy + row as f32 * 43.0;
            let dot = crate::color::wave_color(preset.params.wavelength_m, 1.0);
            if Self::preset_card(
                bx,
                by,
                iw / 2.0 - 5.0,
                preset.name,
                preset.detail,
                dot,
                view,
            ) {
                let observe = sim.observe;
                sim.params = preset.params;
                sim.observe = observe;
                sim.reset();
                out.reset_visuals = true;
            }
        }
        cy += 92.0;

        if in_view(cy, 14.0, view) {
            ui::section_label(ix, cy, "AJUSTES", accent);
        }
        cy += 17.0;
        let p = sim.params;
        let log_frac = |v: f64, min: f64, max: f64| ((v / min).ln() / (max / min).ln()) as f32;
        let log_val = |f: f32, min: f64, max: f64| min * (max / min).powf(f as f64);

        if let Some(f) = self.slider(
            SliderId::Lambda,
            ix,
            cy,
            iw,
            "Longitud de onda λ",
            &if p.wavelength_nm() < 10.0 {
                format!("{:.3} nm", p.wavelength_nm())
            } else {
                format!("{:.0} nm", p.wavelength_nm())
            },
            (p.wavelength_nm() - 380.0) as f32 / 370.0,
            accent,
            view,
        ) {
            sim.params.set_wavelength_nm(380.0 + f as f64 * 370.0);
            sim.reset();
            out.reset_visuals = true;
        }
        cy += 38.0;
        if let Some(f) = self.slider(
            SliderId::Separation,
            ix,
            cy,
            iw,
            "Separación entre rendijas d",
            &format!("{:.1} µm", p.separation_um()),
            log_frac(p.separation_um(), 0.5, 200.0),
            accent,
            view,
        ) {
            sim.params.set_separation_um(log_val(f, 0.5, 200.0));
            sim.reset();
            out.reset_visuals = true;
        }
        cy += 38.0;
        if let Some(f) = self.slider(
            SliderId::Width,
            ix,
            cy,
            iw,
            "Ancho de cada rendija a",
            &format!("{:.1} µm", p.width_um()),
            log_frac(p.width_um(), 0.1, 40.0),
            accent,
            view,
        ) {
            sim.params.set_width_um(log_val(f, 0.1, 40.0));
            sim.reset();
            out.reset_visuals = true;
        }
        cy += 38.0;
        if let Some(f) = self.slider(
            SliderId::Distance,
            ix,
            cy,
            iw,
            "Distancia a la pantalla L",
            &format!("{:.2} m", p.screen_distance_m),
            ((p.screen_distance_m - 0.2) / 2.3) as f32,
            accent,
            view,
        ) {
            sim.params.set_distance_m(0.2 + f as f64 * 2.3);
            sim.reset();
            out.reset_visuals = true;
        }
        cy += 38.0;
        if let Some(f) = self.slider(
            SliderId::Rate,
            ix,
            cy,
            iw,
            "Ritmo de disparo",
            &format!("{} / frame", rate),
            log_frac(*rate as f64, 1.0, 2000.0),
            accent,
            view,
        ) {
            *rate = (log_val(f, 1.0, 2000.0) as u32).max(1);
        }
        cy += 38.0;

        if in_view(cy, 14.0, view) {
            ui::section_label(ix, cy, "RENDIJAS", accent);
        }
        cy += 17.0;
        for (i, &n) in [1usize, 2, 3, 4, 6, 8].iter().enumerate() {
            let bx = ix + i as f32 * (iw / 6.0 + 1.5);
            if Self::button(
                bx,
                cy,
                iw / 6.0 - 3.0,
                24.0,
                &format!("{n}"),
                sim.params.slit_count == n,
                view,
            ) {
                sim.params.set_slit_count(n);
                sim.reset();
                out.reset_visuals = true;
            }
        }
        cy += 36.0;

        if Self::button(ix, cy, iw / 3.0 - 5.0, 28.0, "▶ +100", false, view) {
            out.shoot = 100;
        }
        if Self::button(
            ix + iw / 3.0 + 2.0,
            cy,
            iw / 3.0 - 5.0,
            28.0,
            if *paused { "▶ Seguir" } else { "⏸ Pausa" },
            *paused,
            view,
        ) {
            *paused = !*paused;
        }
        if Self::button(
            ix + 2.0 * iw / 3.0 + 4.0,
            cy,
            iw / 3.0 - 5.0,
            28.0,
            "↺ Reiniciar",
            false,
            view,
        ) {
            sim.reset();
            out.reset_visuals = true;
        }
        cy += 38.0;

        if in_view(cy, 14.0, view) {
            ui::section_label(ix, cy, "CÁMARA", accent);
        }
        cy += 17.0;
        let views = [
            ("Panorama", ViewPreset::Panorama),
            ("Fuente", ViewPreset::Fuente),
            ("Rendijas", ViewPreset::Rendijas),
            ("Pantalla", ViewPreset::Pantalla),
            ("Cenital", ViewPreset::Cenital),
            ("Presentar", ViewPreset::Panorama),
        ];
        for (i, (label, preset)) in views.iter().enumerate() {
            let col = i % 3;
            let row = i / 3;
            let bx = ix + col as f32 * (iw / 3.0 + 2.0);
            let by = cy + row as f32 * 29.0;
            if Self::button(bx, by, iw / 3.0 - 4.0, 24.0, label, false, view) {
                if label == &"Presentar" {
                    *presentation = true;
                } else {
                    camera.apply_preset(*preset);
                }
            }
        }
        cy += 62.0;

        if in_view(cy, 14.0, view) {
            ui::section_label(ix, cy, "CAPAS Y MOVIMIENTO", accent);
        }
        cy += 17.0;
        let toggles: [(&str, &mut bool); 5] = [
            ("Onda", &mut layers.wave),
            ("Haces", &mut layers.beams),
            ("Partículas", &mut layers.particles),
            ("Notas", &mut layers.annotations),
            ("Gráfica", &mut layers.chart),
        ];
        for (i, (label, flag)) in toggles.into_iter().enumerate() {
            let col = i % 3;
            let row = i / 3;
            let bx = ix + col as f32 * (iw / 3.0 + 2.0);
            let by = cy + row as f32 * 29.0;
            let current = *flag;
            if Self::button(bx, by, iw / 3.0 - 4.0, 24.0, label, current, view) {
                *flag = !current;
            }
        }
        cy += 64.0;
        if in_view(cy, 14.0, view) {
            ui::section_label(ix, cy, "RENDIMIENTO", accent);
        }
        cy += 17.0;
        for (i, (label, mode)) in [
            ("Auto", QualityMode::Auto),
            ("Alta", QualityMode::Alta),
            ("Ligera", QualityMode::Ligera),
        ]
        .iter()
        .enumerate()
        {
            let bx = ix + i as f32 * (iw / 3.0 + 2.0);
            if Self::button(
                bx,
                cy,
                iw / 3.0 - 4.0,
                24.0,
                label,
                *quality_mode == *mode,
                view,
            ) {
                *quality_mode = *mode;
            }
        }
        cy += 30.0;
        if in_view(cy, 26.0, view) {
            ui::text(
                &format!(
                    "Calidad activa: {} · pensada para ir fluido sin GPU",
                    effective_quality.label()
                ),
                ix,
                cy + 10.0,
                11.5,
                color::MUTED,
                Weight::Regular,
            );
        }
        cy += 58.0;
        if in_view(cy, 14.0, view) {
            ui::section_label(ix, cy, "CÁMARA LIBRE", accent);
        }
        cy += 17.0;
        if in_view(cy, 26.0, view) {
            ui::text(
                "Cámara libre",
                ix,
                cy + 14.0,
                13.0,
                color::TEXT,
                Weight::SemiBold,
            );
            let (sx, sy, sw, sh) = ui::switch(ix + iw - 36.0, cy, camera.free, color::ACCENT);
            if inside(sx, sy, sw, sh) && is_mouse_button_pressed(MouseButton::Left) {
                camera.free = !camera.free;
            }
        }
        cy += 28.0;
        if in_view(cy, 32.0, view) {
            ui::text("Zoom", ix, cy + 3.0, 13.0, color::MUTED, Weight::Regular);
            ui::badge(
                ix + iw,
                cy + 3.0,
                &format!("{:.1} u", camera.distance_goal()),
                color::ACCENT,
            );
            ui::rounded_rect(
                ix,
                cy + 12.0,
                iw,
                6.0,
                3.0,
                Color::new(0.22, 0.27, 0.37, 1.0),
            );
            let fx = ix + camera.zoom_fraction() * iw;
            if fx > ix {
                ui::rounded_rect(ix, cy + 12.0, fx - ix, 6.0, 3.0, color::ACCENT);
            }
            draw_circle(fx, cy + 15.0, 7.0, WHITE);
            let (mx, my) = mouse_position();
            if mx >= ix - 5.0
                && mx <= ix + iw + 5.0
                && my >= cy + 6.0
                && my <= cy + 24.0
                && is_mouse_button_down(MouseButton::Left)
            {
                camera.set_zoom_fraction((mx - ix) / iw);
            }
        }
        cy += 34.0;
        if in_view(cy, 26.0, view) {
            ui::text(
                "Órbita cinemática lenta",
                ix,
                cy + 14.0,
                13.0,
                color::TEXT,
                Weight::SemiBold,
            );
            let (sx, sy, sw, sh) = ui::switch(ix + iw - 36.0, cy, *cinematic, color::ACCENT);
            if inside(sx, sy, sw, sh) && is_mouse_button_pressed(MouseButton::Left) {
                *cinematic = !*cinematic;
            }
        }

        // Altura real del contenido, sombras de borde y barra.
        let content_h = cy + self.scroll - view.0;
        self.max_scroll = scroll_max(content_h, view_h);
        self.scroll = self.scroll.min(self.max_scroll);
        for (edge_y, alto, a) in [
            (view.0, 7.0, 0.20f32),
            (view.0, 3.0, 0.30),
            (view.1 - 7.0, 7.0, 0.20),
            (view.1 - 3.0, 3.0, 0.30),
        ] {
            let show = if edge_y < view.0 + 1.0 {
                self.scroll > 0.5
            } else {
                self.scroll < self.max_scroll - 0.5
            };
            if show {
                ui::rounded_rect(
                    ix - 6.0,
                    edge_y,
                    iw + 12.0,
                    alto,
                    2.0,
                    Color::new(0.0, 0.0, 0.0, a),
                );
            }
        }
        if self.max_scroll > 0.0 {
            let (thumb_h, thumb_dy) = thumb_geometry(view_h, view_h + self.max_scroll, self.scroll);
            let rail_x = x + w - 8.0;
            ui::rounded_rect(
                rail_x,
                view.0,
                4.0,
                view_h,
                2.0,
                Color::new(1.0, 1.0, 1.0, 0.07),
            );
            let hot = self.scroll_drag.is_some();
            ui::rounded_rect(
                rail_x - if hot { 1.0 } else { 0.0 },
                view.0 + thumb_dy,
                if hot { 6.0 } else { 4.0 },
                thumb_h,
                3.0,
                Color::new(
                    color::ACCENT.r,
                    color::ACCENT.g,
                    color::ACCENT.b,
                    if hot { 0.95 } else { 0.55 },
                ),
            );
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin_desborde_no_hay_scroll() {
        assert_eq!(scroll_max(500.0, 500.0), 0.0);
        assert_eq!(scroll_max(300.0, 500.0), 0.0);
        let (thumb_h, dy) = thumb_geometry(500.0, 400.0, 0.0);
        assert_eq!(thumb_h, 500.0, "el pulgar ocupa toda la vista");
        assert_eq!(dy, 0.0);
    }

    #[test]
    fn el_pulgar_es_proporcional_y_llega_al_fondo() {
        let (thumb_h, dy0) = thumb_geometry(400.0, 800.0, 0.0);
        assert!((thumb_h - 200.0).abs() < 0.5, "mitad de contenido visible");
        assert_eq!(dy0, 0.0);
        let (_, dy_end) = thumb_geometry(400.0, 800.0, 400.0);
        assert!(
            (dy_end + thumb_h - 400.0).abs() < 0.5,
            "al final el pulgar toca el fondo"
        );
    }
}
