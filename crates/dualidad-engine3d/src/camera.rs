//! Cámara orbital libre con zoom y movimiento suaves (amortiguados).
//!
//! La cámara mantiene dos poses: la **actual** (la que se dibuja) y la
//! **objetivo** (la que pide el usuario). Cada frame la actual persigue a
//! la objetivo con amortiguación exponencial independiente de la tasa de
//! fotogramas; así el zoom con rueda, la órbita con arrastre, el panel
//! deslizante, el teclado y los cambios de vista (presets) se sienten
//! fluidos, sin saltos ni acelerones. El zoom se interpola en escala
//! logarítmica para que la velocidad percibida sea constante.
//!
//! Controles (sobre la escena 3D):
//! - Arrastrar con botón izquierdo: orbitar.
//! - Arrastrar con botón derecho o central (o Mayús+izquierdo): desplazar
//!   el centro de interés (pan).
//! - Rueda del ratón: zoom suave y completo, 0.8–32 unidades (del detalle
//!   de una rendija a la vista general del laboratorio).
//! - Alt + rueda: ajustar la lente (campo de visión 20°–70°).
//! - `W`/`A`/`S`/`D`: trasladar el centro (adelante/izquierda/atrás/
//!   derecha); `Q`/`E`: bajar/subir; Mayús: ir rápido.
//! - Doble clic o tecla `C`: restablecer la vista.
//!
//! La cámara nunca baja del suelo ni se voltea en el cenital: la pose
//! se limita para que la posición quede siempre sobre el piso y lejos
//! del polo vertical, donde el «arriba» degenera y la vista salta.
//! (Antes, orbitar hacia abajo metía la cámara bajo el piso y todo se
//! veía negro desde el revés del mundo: la «mancha negra».)
//!
//! El modo «libre» (interruptor del panel) da rango completo de pitch y
//! desplazamiento amplio; el modo guiado limita ambos para mantener
//! siempre el experimento en cuadro, pensado para presentaciones.

use macroquad::prelude::*;

/// Vistas predefinidas de cámara (botones del panel).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewPreset {
    /// Vista general del laboratorio.
    Panorama,
    /// Detalle de la fuente emisora.
    Fuente,
    /// Detalle de la barrera de rendijas.
    Rendijas,
    /// Detalle de la pantalla detectora.
    Pantalla,
    /// Vista superior (cenital).
    Cenital,
}

const DEFAULT_YAW: f32 = -0.20;
const DEFAULT_PITCH: f32 = 0.34;
const DEFAULT_DISTANCE: f32 = 13.5;
const DEFAULT_TARGET: Vec3 = vec3(0.0, 1.5, 0.3);
const DEFAULT_FOVY: f32 = 0.80;

/// Rango de zoom: detalle extremo ↔ panorama completo.
const MIN_DISTANCE: f32 = 0.8;
const MAX_DISTANCE: f32 = 32.0;
/// La cámara nunca baja de esta altura (no atravesar el suelo).
const MIN_CAMERA_Y: f32 = 0.30;
/// Rango de lente (campo de visión vertical, radianes): tele ↔ gran angular.
const MIN_FOVY: f32 = 0.35; // ≈ 20°
const MAX_FOVY: f32 = 1.22; // ≈ 70°

/// Pose esférica interpolable de la cámara.
#[derive(Debug, Clone, Copy)]
struct Pose {
    yaw: f32,
    pitch: f32,
    distance: f32,
    fovy: f32,
    target: Vec3,
}

impl Pose {
    fn clamp(&mut self, free: bool) {
        // Lejos del polo vertical (±π/2): ahí el «arriba» de la cámara
        // degenera y la vista se voltea de golpe.
        let (mut min_pitch, max_pitch): (f32, f32) =
            if free { (-1.45, 1.45) } else { (0.06, 1.35) };
        self.distance = self.distance.clamp(MIN_DISTANCE, MAX_DISTANCE);
        self.fovy = self.fovy.clamp(MIN_FOVY, MAX_FOVY);
        // El centro de interés no puede perderse lejos del experimento.
        let (tx, ty, tz) = if free {
            (14.0, 7.0, 12.0)
        } else {
            (4.0, 3.0, 4.0)
        };
        self.target.x = self.target.x.clamp(-tx, tx);
        self.target.y = self.target.y.clamp(0.05, ty);
        self.target.z = self.target.z.clamp(-tz, tz);
        // Suelo: posición.y = target.y + sin(pitch)·distancia ≥ mínimo.
        let sin_min = ((MIN_CAMERA_Y - self.target.y) / self.distance).clamp(-1.0, 1.0);
        min_pitch = min_pitch.max(sin_min.asin());
        self.pitch = self.pitch.clamp(min_pitch, max_pitch);
    }

    fn position(&self) -> Vec3 {
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();
        self.target
            + vec3(
                sy * cp * self.distance,
                sp * self.distance,
                cy * cp * self.distance,
            )
    }
}

/// Cámara orbital libre con zoom suave.
pub struct OrbitCamera {
    /// Centro de interés actual (ya suavizado).
    pub target: Vec3,
    /// Distancia actual suavizada.
    pub distance: f32,
    /// Campo de visión vertical actual suavizado (radianes).
    pub fovy: f32,
    /// `true`: rango completo; `false`: guiada (experimento siempre en cuadro).
    pub free: bool,
    current: Pose,
    goal: Pose,
    dragging_orbit: bool,
    dragging_pan: bool,
    last_mouse: Vec2,
    last_click_time: f64,
    last_press_pos: Vec2,
    press_pos: Vec2,
}

impl OrbitCamera {
    pub fn new() -> Self {
        let pose = Pose {
            yaw: DEFAULT_YAW,
            pitch: DEFAULT_PITCH,
            distance: DEFAULT_DISTANCE,
            fovy: DEFAULT_FOVY,
            target: DEFAULT_TARGET,
        };
        Self {
            target: DEFAULT_TARGET,
            distance: DEFAULT_DISTANCE,
            fovy: DEFAULT_FOVY,
            free: true,
            current: pose,
            goal: pose,
            dragging_orbit: false,
            dragging_pan: false,
            last_mouse: Vec2::ZERO,
            last_click_time: -10.0,
            last_press_pos: Vec2::ZERO,
            press_pos: Vec2::ZERO,
        }
    }

    /// Vuelve a la vista por defecto (con vuelo animado).
    pub fn reset(&mut self) {
        self.goal = Pose {
            yaw: DEFAULT_YAW,
            pitch: DEFAULT_PITCH,
            distance: DEFAULT_DISTANCE,
            fovy: DEFAULT_FOVY,
            target: DEFAULT_TARGET,
        };
    }

    /// Posición actual de la cámara en el mundo.
    pub fn position(&self) -> Vec3 {
        self.current.position()
    }

    pub fn to_camera3d(&self) -> Camera3D {
        Camera3D {
            position: self.position(),
            up: vec3(0.0, 1.0, 0.0),
            target: self.current.target,
            fovy: self.current.fovy,
            ..Default::default()
        }
    }

    /// Aplica una vista predefinida como objetivo: la cámara vuela hacia ella.
    pub fn apply_preset(&mut self, preset: ViewPreset) {
        let (yaw, pitch, dist, target) = match preset {
            ViewPreset::Panorama => (-0.62, 0.26, 24.0, vec3(0.0, 1.6, 0.3)),
            ViewPreset::Fuente => (0.55, 0.30, 5.0, vec3(0.0, 1.7, -5.2)),
            ViewPreset::Rendijas => (0.60, 0.28, 4.2, vec3(0.0, 1.7, 0.0)),
            // Delante de la pantalla (entre las rendijas y la pantalla):
            // con yaw ≈ π la cámara queda en z < pantalla mirándola de
            // frente. Antes quedaba DETRÁS y solo se veía su dorso negro.
            ViewPreset::Pantalla => (2.79, 0.22, 5.2, vec3(0.0, 1.7, 5.9)),
            ViewPreset::Cenital => (0.0, 1.45, 16.0, vec3(0.0, 0.9, 0.3)),
        };
        self.goal.yaw = yaw;
        self.goal.pitch = pitch;
        self.goal.distance = dist;
        self.goal.target = target;
    }

    /// Orbita una cantidad en yaw (órbita cinemática del modo presentación).
    pub fn orbit_by(&mut self, dyaw: f32) {
        self.goal.yaw += dyaw;
        self.goal.clamp(self.free);
    }

    /// Traslada el centro de interés con el teclado (WASD/QE).
    /// `fwd`/`right`/`up` ∈ {-1, 0, 1}: adelante/lateral en el plano
    /// horizontal según hacia dónde se mira, y altura. La velocidad
    /// crece con la distancia: de cerca se mueve fino y de lejos se
    /// cruza el laboratorio enseguida. `boost` (Mayús) multiplica ×3.
    pub fn move_wasd(&mut self, dt: f32, fwd: f32, right: f32, up: f32, boost: bool) {
        let rapidez = if boost { 3.0 } else { 1.0 };
        let speed = (self.goal.distance * 0.85).max(1.0) * rapidez;
        let (sy, cy) = self.goal.yaw.sin_cos();
        let forward = vec3(-sy, 0.0, -cy);
        let right_v = vec3(cy, 0.0, -sy);
        self.goal.target += (forward * fwd + right_v * right) * speed * dt;
        self.goal.target.y += up * speed * dt;
        self.goal.clamp(self.free);
    }

    /// Detiene cualquier arrastre en curso (p. ej. al pasar sobre el panel).
    pub fn cancel_drag(&mut self) {
        self.dragging_orbit = false;
        self.dragging_pan = false;
    }

    /// Distancia objetivo (el zoom pedido, sin suavizar).
    pub fn distance_goal(&self) -> f32 {
        self.goal.distance
    }

    /// Fracción de zoom 0..=1 en escala logarítmica (para el deslizador).
    pub fn zoom_fraction(&self) -> f32 {
        (self.goal.distance.ln() - MIN_DISTANCE.ln()) / (MAX_DISTANCE.ln() - MIN_DISTANCE.ln())
    }

    /// Fija el zoom desde la fracción 0..=1 (escala logarítmica).
    pub fn set_zoom_fraction(&mut self, fraction: f32) {
        let f = fraction.clamp(0.0, 1.0);
        self.goal.distance =
            (MIN_DISTANCE.ln() + f * (MAX_DISTANCE.ln() - MIN_DISTANCE.ln())).exp();
    }

    /// Desplaza el centro de interés en el plano de la cámara.
    fn pan(&mut self, dx: f32, dy: f32) {
        let forward = (self.current.target - self.current.position()).normalize_or_zero();
        let right = forward.cross(vec3(0.0, 1.0, 0.0)).normalize_or_zero();
        let up = right.cross(forward).normalize_or_zero();
        let scale = self.goal.distance * 0.0016;
        self.goal.target += (-right * dx + up * dy) * scale;
        self.goal.clamp(self.free);
    }

    /// Lee el ratón: órbita, desplazamiento, rueda (zoom/lente) y doble clic.
    fn poll_mouse(&mut self) {
        let mouse = mouse_position();
        let pos = vec2(mouse.0, mouse.1);
        if is_mouse_button_pressed(MouseButton::Left) {
            // Doble clic: restablecer la vista. Solo cuenta si el
            // segundo clic cae casi sobre el primero y entre ambos no
            // hubo arrastre (dos clics para orbitar no deben reiniciar).
            let now = get_time();
            if now - self.last_click_time < 0.35 && (pos - self.last_press_pos).length() < 6.0 {
                self.reset();
                self.last_click_time = -10.0;
            } else {
                self.last_click_time = now;
                self.last_press_pos = pos;
            }
            self.press_pos = pos;
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.dragging_pan = true;
            } else {
                self.dragging_orbit = true;
            }
            self.last_mouse = pos;
        }
        if is_mouse_button_pressed(MouseButton::Right)
            || is_mouse_button_pressed(MouseButton::Middle)
        {
            self.dragging_pan = true;
            self.last_mouse = pos;
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.dragging_orbit = false;
            self.dragging_pan = false;
            // Si hubo arrastre, el siguiente clic no es «doble clic».
            if (pos - self.press_pos).length() > 6.0 {
                self.last_click_time = -10.0;
            }
        }
        if is_mouse_button_released(MouseButton::Right)
            || is_mouse_button_released(MouseButton::Middle)
        {
            self.dragging_pan = false;
        }
        if self.dragging_orbit || self.dragging_pan {
            let delta = pos - self.last_mouse;
            self.last_mouse = pos;
            if self.dragging_orbit {
                self.goal.yaw -= delta.x * 0.0052;
                self.goal.pitch += delta.y * 0.0052;
                self.goal.clamp(self.free);
            } else {
                self.pan(delta.x, delta.y);
            }
        }
        // Rueda: zoom exponencial suave; con Alt, lente.
        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            if is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt) {
                self.goal.fovy = (self.goal.fovy * (-wheel * 0.06).exp()).clamp(MIN_FOVY, MAX_FOVY);
            } else {
                // Cada «paso» de rueda multiplica la distancia por ≈0.88.
                self.goal.distance =
                    (self.goal.distance * (-wheel * 0.13).exp()).clamp(MIN_DISTANCE, MAX_DISTANCE);
            }
        }
    }

    /// Avanza un frame: lee el ratón si `allow_input` y suaviza la pose.
    /// `dt` en segundos (se capa a 50 ms para pestañeos de foco).
    pub fn update(&mut self, dt: f32, allow_input: bool) {
        if allow_input {
            self.poll_mouse();
        } else {
            self.cancel_drag();
        }
        // Si se apaga el modo libre, la pose objetivo se reencuadra sola.
        self.goal.clamp(self.free);
        let dt = dt.clamp(0.0, 0.05);
        // Constantes de persecución: rotación ágil; zoom/lente más suaves.
        let ka = 1.0 - (-dt * 13.0).exp();
        let kz = 1.0 - (-dt * 9.0).exp();
        // Yaw por el camino más corto alrededor del círculo.
        let mut dyaw = self.goal.yaw - self.current.yaw;
        while dyaw > std::f32::consts::PI {
            dyaw -= std::f32::consts::TAU;
        }
        while dyaw < -std::f32::consts::PI {
            dyaw += std::f32::consts::TAU;
        }
        self.current.yaw += dyaw * ka;
        self.current.pitch += (self.goal.pitch - self.current.pitch) * ka;
        // Zoom en escala logarítmica: velocidad perceptiva constante.
        let log_cur = self.current.distance.ln();
        let log_goal = self.goal.distance.ln();
        self.current.distance = (log_cur + (log_goal - log_cur) * kz).exp();
        self.current.fovy += (self.goal.fovy - self.current.fovy) * kz;
        self.current.target += (self.goal.target - self.current.target) * kz;
        self.current.clamp(self.free);
        // Publicar estado instantáneo.
        self.target = self.current.target;
        self.distance = self.current.distance;
        self.fovy = self.current.fovy;
    }
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_distance_is_overview() {
        let cam = OrbitCamera::new();
        assert!((cam.distance - 13.5).abs() < 0.01);
        assert!(cam.position().length() > 1.0);
    }

    #[test]
    fn zoom_is_smooth_not_instant() {
        let mut cam = OrbitCamera::new();
        cam.goal.distance = 3.0;
        cam.update(1.0 / 60.0, false);
        // Tras un frame la distancia bajó, pero no llegó al objetivo.
        assert!(cam.distance < 13.4 && cam.distance > 3.5);
    }

    #[test]
    fn zoom_converges_and_clamps() {
        let mut cam = OrbitCamera::new();
        cam.set_zoom_fraction(0.0);
        assert!((cam.distance_goal() - MIN_DISTANCE).abs() < 0.01);
        cam.set_zoom_fraction(1.0);
        assert!((cam.distance_goal() - MAX_DISTANCE).abs() < 0.01);
        for _ in 0..600 {
            cam.update(1.0 / 60.0, false);
        }
        assert!((cam.distance - MAX_DISTANCE).abs() < 0.05);
    }

    #[test]
    fn la_camara_nunca_baja_del_suelo() {
        let mut cam = OrbitCamera::new();
        cam.goal.pitch = -1.45; // orbitar «hacia abajo» a fondo
        for _ in 0..600 {
            cam.update(1.0 / 60.0, false);
        }
        assert!(
            cam.position().y >= MIN_CAMERA_Y - 0.01,
            "la cámara quedó bajo el suelo: y={}",
            cam.position().y
        );
    }

    #[test]
    fn el_preset_pantalla_queda_delante_de_la_pantalla() {
        let mut cam = OrbitCamera::new();
        cam.apply_preset(ViewPreset::Pantalla);
        for _ in 0..600 {
            cam.update(1.0 / 60.0, false);
        }
        assert!(
            cam.position().z < crate::layout::SCREEN_Z - 0.5,
            "la cámara debe mirar la pantalla desde delante, z={}",
            cam.position().z
        );
    }

    #[test]
    fn wasd_mueve_el_centro_hacia_donde_se_mira() {
        let mut cam = OrbitCamera::new();
        let antes = cam.goal.target;
        cam.move_wasd(1.0, 1.0, 0.0, 0.0, false); // W durante 1 s
        assert!(cam.goal.target.z < antes.z, "W debe acercar al fondo (-z)");
        assert!((cam.goal.target.y - antes.y).abs() < 0.001, "W no sube");
        let antes = cam.goal.target;
        cam.move_wasd(1.0, 0.0, 0.0, 1.0, false); // E durante 1 s
        assert!(cam.goal.target.y > antes.y, "E debe subir el centro");
    }

    #[test]
    fn reset_restores_defaults() {
        let mut cam = OrbitCamera::new();
        cam.apply_preset(ViewPreset::Cenital);
        cam.reset();
        assert!((cam.distance_goal() - 13.5).abs() < 0.01);
        assert!((cam.goal.pitch - 0.34).abs() < 0.001);
    }
}
