//! Motor de escena 3D del simulador (macroquad).
//!
//! Responsabilidades de este crate:
//! - la cámara orbital y la escena del laboratorio (fuente, barrera con
//!   dos rendijas, pantalla),
//! - la superficie 3D de la distribución `I(x)` entre las rendijas y la
//!   pantalla,
//! - los impactos como malla eficiente sobre la pantalla 3D,
//! - el histograma en panel 2D comparado con la curva teórica,
//! - partículas en vuelo como recurso visual (trayectorias esquemáticas),
//! - el HUD 2D superpuesto.
//!
//! No contiene física: todo el modelo vive en `dualidad-core`.

pub mod annotations;
pub mod beams;
pub mod camera;
pub mod color;
pub mod controls;
pub mod hud;
pub mod impacts;
pub mod particles;
pub mod quality;
pub mod renderer;
pub mod scene;
pub mod ui;
pub mod wave_surface;

pub use camera::{OrbitCamera, ViewPreset};
pub use controls::{ControlOutcome, ControlPanel, Layers};
pub use hud::HudInfo;
pub use quality::{QualityMode, QualityTier};
pub use renderer::Renderer;

/// Diseño espacial de la escena (unidades del mundo 3D).
pub mod layout {
    /// Eje Z: fuente → rendijas → pantalla.
    pub const SOURCE_Z: f32 = -5.5;
    pub const SLITS_Z: f32 = 0.0;
    pub const SCREEN_Z: f32 = 6.0;
    /// Altura del suelo y del centro óptico.
    pub const FLOOR_Y: f32 = 0.0;
    pub const OPTICAL_AXIS_Y: f32 = 1.7;
    /// Ancho del mundo que representa la pantalla (±20 mm).
    pub const SCREEN_WIDTH: f32 = 8.0;
    pub const SCREEN_HEIGHT: f32 = 2.7;

    /// Convierte `u ∈ [0, 1]` (pantalla normalizada) a X del mundo 3D.
    pub fn u_to_world_x(u: f64) -> f32 {
        (u as f32 - 0.5) * SCREEN_WIDTH
    }

    /// Altura Y del mundo donde un impacto con `v ∈ [0, 1]` toca la
    /// pantalla. La usan igual los impactos dibujados y las partículas
    /// en vuelo: si difirieran, las partículas «llegarían» a otro sitio
    /// (estaban 0.35 u por debajo y se perdían tras el marco inferior).
    pub fn impact_y(v: f64) -> f32 {
        OPTICAL_AXIS_Y - SCREEN_HEIGHT / 2.0 + 0.08 + v as f32 * (SCREEN_HEIGHT - 0.16)
    }
}
