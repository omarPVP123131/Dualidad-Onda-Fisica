//! Anotaciones 2D proyectadas sobre la escena 3D: nombre de cada
//! elemento con su línea guía, como en un diagrama técnico.

use crate::camera::OrbitCamera;
use crate::color;
use crate::layout;
use crate::ui::{self, Weight};
use macroquad::prelude::*;

/// Proyecta un punto del mundo a píxeles del área 3D (viewport, sin el
/// dock lateral). `None` si queda tras la cámara.
fn project(camera: &OrbitCamera, viewport_w: f32, point: Vec3) -> Option<Vec2> {
    let view = Mat4::look_at_rh(camera.position(), camera.target, vec3(0.0, 1.0, 0.0));
    let aspect = viewport_w / screen_height().max(1.0);
    let proj = Mat4::perspective_rh_gl(camera.fovy, aspect, 0.01, 1_000.0);
    let clip = proj * view * point.extend(1.0);
    if clip.w <= 0.0 {
        return None;
    }
    let ndc = clip.truncate() / clip.w;
    Some(vec2(
        (ndc.x + 1.0) * 0.5 * viewport_w,
        (1.0 - ndc.y) * 0.5 * screen_height(),
    ))
}

fn label(camera: &OrbitCamera, viewport_w: f32, point: Vec3, text: &str, dx: f32, dy: f32) {
    let Some(p) = project(camera, viewport_w, point) else {
        return;
    };
    let lx = (p.x + dx).clamp(8.0, (viewport_w - 210.0).max(8.0));
    let ly = (p.y + dy).clamp(8.0, screen_height() - 110.0);
    draw_circle(p.x, p.y, 4.5, Color::new(0.0, 0.0, 0.0, 0.4));
    draw_circle(p.x, p.y, 3.0, color::TEXT);
    draw_line(p.x, p.y, lx + 6.0, ly + 11.0, 1.0, color::PANEL_EDGE);
    let w = ui::measure(text, 12.5, Weight::SemiBold) + 18.0;
    ui::rounded_rect(lx, ly, w, 22.0, 8.0, color::PANEL);
    ui::text(
        text,
        lx + 9.0,
        ly + 15.5,
        12.5,
        color::TEXT,
        Weight::SemiBold,
    );
}

pub fn draw_annotations(camera: &OrbitCamera, viewport_w: f32) {
    let axis = layout::OPTICAL_AXIS_Y;
    label(
        camera,
        viewport_w,
        vec3(0.0, axis + 0.35, layout::SOURCE_Z),
        "FUENTE",
        -118.0,
        -44.0,
    );
    label(
        camera,
        viewport_w,
        vec3(
            0.0,
            axis + layout::SCREEN_HEIGHT / 2.0 + 0.45,
            layout::SLITS_Z,
        ),
        "DOBLE RENDIJA · DETECTOR",
        30.0,
        -18.0,
    );
    label(
        camera,
        viewport_w,
        vec3(-2.6, 2.15, 3.4),
        "SUPERFICIE DE PROBABILIDAD I(x)",
        -150.0,
        -34.0,
    );
    label(
        camera,
        viewport_w,
        vec3(
            2.7,
            axis + layout::SCREEN_HEIGHT / 2.0 + 0.3,
            layout::SCREEN_Z,
        ),
        "PANTALLA",
        24.0,
        -40.0,
    );
}
