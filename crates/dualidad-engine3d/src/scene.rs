//! La escena del laboratorio: banco óptico, fuente, barrera con dos
//! rendijas, sensores del detector y pantalla.
//!
//! Las distancias transversales son **esquemáticas**: a escala real
//! (rendijas de µm frente a una pantalla de 40 mm) nada se vería. La
//! separación y el ancho de las rendijas se exagera con una ley lineal
//! saturada, solo para representar, nunca para medir.

use crate::color;
use crate::layout;
use dualidad_core::SimulationParams;
use macroquad::prelude::*;

/// Centros visuales de las `slit_count` rendijas en X del mundo.
pub fn slit_centers_x(params: &SimulationParams) -> Vec<f32> {
    let n = params.slit_count.max(1);
    let sep = visual_separation(params);
    let mid = (n as f32 - 1.0) / 2.0;
    (0..n).map(|i| (i as f32 - mid) * sep).collect()
}

/// Separación visual entre rendijas (exagerada, saturada).
pub fn visual_separation(params: &SimulationParams) -> f32 {
    (params.slit_separation_m as f32 * 8_000.0).clamp(0.3, 2.4)
}

/// Ancho visual de cada rendija (exagerado, saturado).
pub fn visual_slit_width(params: &SimulationParams) -> f32 {
    (params.slit_width_m as f32 * 9_000.0).clamp(0.07, 0.36)
}

/// Bloques de pared (centro, ancho) que quedan al abrir las dos rendijas.
fn wall_blocks(params: &SimulationParams) -> Vec<(f32, f32)> {
    let w = layout::SCREEN_WIDTH - 0.4;
    let s = visual_slit_width(params) / 2.0;
    let mut cuts: Vec<f32> = vec![-w / 2.0];
    for c in slit_centers_x(params) {
        cuts.push(c - s);
        cuts.push(c + s);
    }
    cuts.push(w / 2.0);
    let mut blocks = Vec::new();
    for pair in cuts.chunks(2) {
        if pair.len() == 2 {
            let width = pair[1] - pair[0];
            if width > 0.01 {
                blocks.push(((pair[0] + pair[1]) / 2.0, width));
            }
        }
    }
    blocks
}

/// Marco luminoso mínimo alrededor de cada rendija (sin él, las rendijas
/// a esta escala son invisibles y la barrera parece una pared ciega).
fn draw_slit_frames(params: &SimulationParams) {
    let glow = color::wave_color(params.wavelength_m, 0.95);
    let w = visual_slit_width(params);
    let h = layout::SCREEN_HEIGHT - 0.7;
    let y = layout::OPTICAL_AXIS_Y;
    let z = layout::SLITS_Z - 0.11;
    for cx in slit_centers_x(params) {
        // Dos montantes verticales tenues delimitando la abertura.
        for side in [-1.0f32, 1.0] {
            draw_cube(
                vec3(cx + side * (w / 2.0 + 0.015), y, z),
                vec3(0.03, h, 0.03),
                None,
                glow,
            );
        }
    }
}

pub fn draw_lab(sim_observe: bool, params: &SimulationParams, ambient_detail: bool) {
    let axis_y = layout::OPTICAL_AXIS_Y;
    let wave = color::wave_color(params.wavelength_m, 1.0);

    // Estrellas de polvo lejanas sobre el fondo (deterministas).
    if ambient_detail {
        let mut seed = 0x2545_u32;
        let mut rnd = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / 16_777_216.0
        };
        for _ in 0..80 {
            let x = -10.0 + rnd() * 20.0;
            let y = 0.4 + rnd() * 6.5;
            draw_cube(
                vec3(x, y, 8.45),
                vec3(0.035, 0.035, 0.01),
                None,
                Color::new(0.6, 0.7, 0.9, 0.28),
            );
        }
    }
    // Resplandor del haz sobre el suelo.
    if ambient_detail {
        draw_plane(
            vec3(0.0, 0.018, 0.3),
            vec2(9.5, 13.5),
            None,
            color::wave_color(params.wavelength_m, 0.045),
        );
    }

    // --- Fondo de la sala y suelo ---
    draw_cube(
        vec3(0.0, 3.4, 8.6),
        vec3(22.0, 9.0, 0.2),
        None,
        Color::new(0.050, 0.066, 0.105, 1.0),
    );
    draw_cube(
        vec3(0.0, -0.07, 0.4),
        vec3(15.0, 0.14, 15.5),
        None,
        color::FLOOR,
    );
    // Sombras falsas suaves bajo cada componente (sin motor de luz).
    let sombra = Color::new(0.0, 0.0, 0.0, 0.35);
    for (sx, sz, w, d) in [
        (0.0, layout::SOURCE_Z, 1.1, 1.2),
        (0.0, layout::SLITS_Z, 8.2, 0.8),
        (0.0, layout::SCREEN_Z, 8.8, 0.9),
    ] {
        draw_cube(vec3(sx, 0.012, sz), vec3(w, 0.012, d), None, sombra);
    }
    for i in -7..=7 {
        let x = i as f32;
        draw_line_3d(vec3(x, 0.005, -6.8), vec3(x, 0.005, 7.6), color::GRID);
        let z = i as f32;
        draw_line_3d(vec3(-7.2, 0.005, z), vec3(7.2, 0.005, z), color::GRID);
    }

    // --- Banco óptico: dos rieles, travesaños y regla graduada ---
    for rx in [-0.38, 0.38] {
        draw_cube(
            vec3(rx, 0.10, 0.3),
            vec3(0.16, 0.18, 13.6),
            None,
            color::METAL,
        );
    }
    for i in 0..7 {
        let z = -6.0 + i as f32 * 2.1;
        draw_cube(
            vec3(0.0, 0.055, z),
            vec3(1.15, 0.09, 0.30),
            None,
            color::WALL,
        );
        draw_cube(
            vec3(0.62, 0.16, z),
            vec3(0.16, 0.03, 0.03),
            None,
            color::WALL_EDGE,
        );
    }
    // Placas base de cada componente.
    for z in [layout::SOURCE_Z, layout::SLITS_Z, layout::SCREEN_Z] {
        draw_cube(vec3(0.0, 0.23, z), vec3(1.0, 0.08, 0.9), None, color::METAL);
    }

    // --- Fuente ---
    let source = vec3(0.0, axis_y, layout::SOURCE_Z);
    draw_cylinder(
        vec3(0.0, axis_y / 2.0, layout::SOURCE_Z),
        0.07,
        0.10,
        axis_y,
        None,
        color::METAL,
    );
    draw_cube(source, vec3(0.42, 0.42, 0.52), None, color::WALL);
    draw_cube_wires(source, vec3(0.42, 0.42, 0.52), color::WALL_EDGE);
    draw_sphere(source + vec3(0.0, 0.0, 0.34), 0.11, None, wave);
    for (r, a) in [(0.17, 0.55), (0.25, 0.30)] {
        let mut prev: Option<Vec3> = None;
        for s in 0..=30 {
            let ang = s as f32 / 30.0 * std::f32::consts::TAU;
            let cur = source + vec3(ang.cos() * r, ang.sin() * r, 0.37);
            if let Some(p0) = prev {
                draw_line_3d(p0, cur, Color::new(wave.r, wave.g, wave.b, a));
            }
            prev = Some(cur);
        }
    }
    // Haz incidente (guía visual fina, no una trayectoria medida).
    draw_cube(
        vec3(
            0.0,
            axis_y,
            (layout::SOURCE_Z + layout::SLITS_Z) / 2.0 + 0.2,
        ),
        vec3(0.045, 0.045, layout::SLITS_Z - layout::SOURCE_Z - 0.4),
        None,
        color::wave_color(params.wavelength_m, 0.35),
    );

    // --- Barrera con dos rendijas ---
    for (cx, width) in wall_blocks(params) {
        let size = vec3(width, layout::SCREEN_HEIGHT, 0.16);
        let pos = vec3(cx, axis_y, layout::SLITS_Z);
        draw_cube(pos, size, None, color::WALL);
        draw_cube_wires(pos, size, color::WALL_EDGE);
    }
    // Degradado frontal fingido (sin motor de luz: 5 bandas por bloque).
    for (cx, width) in wall_blocks(params) {
        for b in 0..5 {
            let f = b as f32 / 4.0;
            let band_h = layout::SCREEN_HEIGHT / 5.0;
            let y = axis_y - layout::SCREEN_HEIGHT / 2.0 + band_h * (b as f32 + 0.5);
            let shade = Color::new(0.10 + 0.07 * f, 0.12 + 0.07 * f, 0.17 + 0.09 * f, 1.0);
            draw_cube(
                vec3(cx, y, layout::SLITS_Z - 0.095),
                vec3(width, band_h, 0.012),
                None,
                shade,
            );
        }
    }
    draw_cube(
        vec3(
            0.0,
            axis_y + layout::SCREEN_HEIGHT / 2.0 + 0.05,
            layout::SLITS_Z,
        ),
        vec3(layout::SCREEN_WIDTH - 0.2, 0.10, 0.30),
        None,
        color::METAL,
    );
    draw_slit_frames(params);
    // Patas de la barrera.
    for x in [-3.3, 3.3] {
        draw_cube(
            vec3(
                x,
                axis_y - layout::SCREEN_HEIGHT / 2.0 - 0.35,
                layout::SLITS_Z,
            ),
            vec3(0.18, 0.7, 0.3),
            None,
            color::METAL,
        );
    }
    // Sensores del detector: cuerpo + lente + luz de estado, integrados.
    let status = if sim_observe {
        color::SENSOR_ON
    } else {
        color::SENSOR_OFF
    };
    for cx in slit_centers_x(params) {
        let top = axis_y + layout::SCREEN_HEIGHT / 2.0;
        let body = vec3(cx, top + 0.16, layout::SLITS_Z);
        draw_cube(body, vec3(0.34, 0.2, 0.34), None, color::METAL);
        draw_cylinder(
            body - vec3(0.0, 0.05, 0.24),
            0.055,
            0.055,
            0.16,
            None,
            color::WALL_EDGE,
        );
        draw_sphere(body + vec3(0.0, 0.14, 0.0), 0.045, None, status);
    }

    // --- Modulo electronico del detector, sobre la barrera ---
    // (Integrado en el aparato: una caja suelta en el suelo estorbaba.)
    let centers = slit_centers_x(params);
    let span = centers.last().copied().unwrap_or(0.0) - centers.first().copied().unwrap_or(0.0);
    let top_y = axis_y + layout::SCREEN_HEIGHT / 2.0 + 0.34;
    draw_cube(
        vec3(0.0, top_y, layout::SLITS_Z),
        vec3(span + 1.1, 0.16, 0.4),
        None,
        color::METAL,
    );
    draw_sphere(
        vec3(-0.18, top_y + 0.12, layout::SLITS_Z),
        0.045,
        None,
        status,
    );
    draw_sphere(
        vec3(0.18, top_y + 0.12, layout::SLITS_Z),
        0.045,
        None,
        status,
    );

    // --- Pantalla de detección ---
    let screen_pos = vec3(0.0, axis_y, layout::SCREEN_Z);
    let frame_size = vec3(
        layout::SCREEN_WIDTH + 0.35,
        layout::SCREEN_HEIGHT + 0.35,
        0.16,
    );
    draw_cube(screen_pos, frame_size, None, color::WALL);
    draw_cube(
        screen_pos - vec3(0.0, 0.0, 0.10),
        vec3(layout::SCREEN_WIDTH, layout::SCREEN_HEIGHT, 0.05),
        None,
        color::SCREEN,
    );
    draw_cube_wires(screen_pos, frame_size, color::WALL_EDGE);
    // Marco luminoso y lineas de escaneo de la pantalla.
    {
        let glow = color::wave_color(params.wavelength_m, 0.5);
        let zf = layout::SCREEN_Z - 0.16;
        let (hw, hh) = (layout::SCREEN_WIDTH / 2.0, layout::SCREEN_HEIGHT / 2.0);
        for (a, b) in [
            (vec3(-hw, axis_y - hh, zf), vec3(hw, axis_y - hh, zf)),
            (vec3(hw, axis_y - hh, zf), vec3(hw, axis_y + hh, zf)),
            (vec3(hw, axis_y + hh, zf), vec3(-hw, axis_y + hh, zf)),
            (vec3(-hw, axis_y + hh, zf), vec3(-hw, axis_y - hh, zf)),
        ] {
            draw_line_3d(a, b, glow);
        }
        for i in 1..if ambient_detail { 9 } else { 1 } {
            let y = axis_y - hh + layout::SCREEN_HEIGHT * i as f32 / 9.0;
            draw_line_3d(
                vec3(-hw, y, zf),
                vec3(hw, y, zf),
                Color::new(1.0, 1.0, 1.0, 0.045),
            );
        }
    }
    draw_cube(
        vec3(
            0.0,
            axis_y + layout::SCREEN_HEIGHT / 2.0 + 0.22,
            layout::SCREEN_Z,
        ),
        vec3(layout::SCREEN_WIDTH + 0.1, 0.10, 0.30),
        None,
        color::METAL,
    );
    // Patas de la pantalla.
    for x in [-3.3, 3.3] {
        draw_cube(
            vec3(
                x,
                axis_y - layout::SCREEN_HEIGHT / 2.0 - 0.35,
                layout::SCREEN_Z,
            ),
            vec3(0.18, 0.7, 0.3),
            None,
            color::METAL,
        );
    }
    // Cruz central tenue sobre la pantalla (referencia del eje óptico).
    let zf = layout::SCREEN_Z - 0.14;
    draw_line_3d(
        vec3(-layout::SCREEN_WIDTH / 2.0, axis_y, zf),
        vec3(layout::SCREEN_WIDTH / 2.0, axis_y, zf),
        Color::new(0.25, 0.30, 0.40, 1.0),
    );
    draw_line_3d(
        vec3(0.0, axis_y - layout::SCREEN_HEIGHT / 2.0, zf),
        vec3(0.0, axis_y + layout::SCREEN_HEIGHT / 2.0, zf),
        Color::new(0.25, 0.30, 0.40, 1.0),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn siempre_quedan_tres_bloques_de_pared() {
        let p = SimulationParams::default();
        assert_eq!(wall_blocks(&p).len(), 3);
    }

    #[test]
    fn las_rendijas_son_simetricas() {
        let p = SimulationParams::default();
        let c = slit_centers_x(&p);
        assert_eq!(c.len(), 2);
        assert!((c[0] + c[1]).abs() < 1e-6 && c[0] < 0.0 && c[1] > 0.0);
        let mut p4 = p;
        p4.slit_count = 4;
        let c4 = slit_centers_x(&p4);
        assert_eq!(c4.len(), 4);
        assert!((c4[0] + c4[3]).abs() < 1e-6);
    }
}
