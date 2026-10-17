//! Orquestador del render: cámara 3D → escena → datos → HUD 2D.

use crate::annotations;
use crate::beams::build_beams;
use crate::camera::OrbitCamera;
use crate::controls::{ControlOutcome, ControlPanel, Layers};
use crate::hud::{draw_hud, HudInfo};
use crate::impacts::ImpactsCache;
use crate::particles::FlightParticles;
use crate::quality::{AutoQuality, QualityMode, QualityTier};
use crate::scene;
use crate::wave_surface::{
    build_wave_meshes, draw_profile_glow, draw_wave_contours, draw_wavefronts,
};
use dualidad_core::{Impact, Simulation};
use macroquad::prelude::*;

/// Clave para saber si la superficie de onda debe reconstruirse.
#[derive(PartialEq)]
struct WaveKey {
    wavelength: u64,
    separation: u64,
    width: u64,
    observe: bool,
    slits: usize,
    quality: QualityTier,
}

impl WaveKey {
    fn of(sim: &Simulation) -> Self {
        Self {
            wavelength: sim.params.wavelength_m.to_bits(),
            separation: sim.params.slit_separation_m.to_bits(),
            width: sim.params.slit_width_m.to_bits(),
            observe: sim.observe,
            slits: sim.params.slit_count,
            quality: QualityTier::Alta, // se sobrescribe en render
        }
    }
}

#[derive(Default)]
pub struct Renderer {
    pub camera: OrbitCamera,
    pub layers: Layers,
    pub quality_mode: QualityMode,
    auto_quality: AutoQuality,
    pub cinematic: bool,
    pub presentation: bool,
    panel: ControlPanel,
    render_target: Option<RenderTarget>,
    render_target_key: (u32, u32, QualityTier),
    wave_meshes: Option<Vec<Mesh>>,
    beam_meshes: Option<Vec<Mesh>>,
    wave_key: Option<WaveKey>,
    impacts: ImpactsCache,
    particles: FlightParticles,
}

impl Renderer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Entrada de cámara (llamar antes de renderizar).
    pub fn update_camera(&mut self, dt: f32) {
        if self.cinematic {
            self.camera.orbit_by(dt * 0.07);
        }
        self.camera.update(dt, !self.pointer_over_ui());
    }

    /// Nivel de detalle que se está usando este frame.
    pub fn effective_quality(&self) -> QualityTier {
        match self.quality_mode {
            QualityMode::Alta => QualityTier::Alta,
            QualityMode::Ligera => QualityTier::Ligera,
            QualityMode::Auto => self.auto_quality.tier,
        }
    }

    /// ¿El ratón está sobre el panel de control? (para no orbitar).
    pub fn pointer_over_ui(&self) -> bool {
        !self.presentation && self.panel.pointer_over()
    }

    pub fn cancel_camera_drag(&mut self) {
        self.camera.cancel_drag();
    }

    /// Notifica impactos recién generados (para las partículas en vuelo).
    pub fn notify_impacts(&mut self, sim: &Simulation, new_impacts: &[Impact]) {
        self.particles.spawn(
            &sim.params,
            new_impacts,
            self.effective_quality().particles_cap(),
        );
    }

    pub fn reset_visuals(&mut self) {
        self.particles.clear();
        self.wave_key = None;
    }

    /// Dibuja un frame completo: escena 3D y, encima, el HUD 2D.
    /// Ancho útil para la escena (la ventana menos el dock lateral).
    pub fn viewport_w(&self) -> f32 {
        (screen_width() - crate::controls::SIDEBAR_W - 24.0).max(640.0)
    }

    pub fn render(
        &mut self,
        sim: &mut Simulation,
        hud: &HudInfo,
        dt: f32,
        rate: &mut u32,
        paused: &mut bool,
    ) -> ControlOutcome {
        if self.quality_mode == QualityMode::Auto {
            self.auto_quality.update(dt);
        }
        let tier = self.effective_quality();
        let vw = self.viewport_w();
        let vh = screen_height();
        let mut cam3d = self.camera.to_camera3d();
        cam3d.aspect = Some(vw / vh.max(1.0));
        // Resolucion dinamica: la escena se dibuja a un destino y se
        // escala a pantalla; la UI queda nitida encima. El destino se
        // usa SIEMPRE (en Alta, a escala 1.0): el camino «directo» con
        // viewport fijo tardaba decenas de frames en aparecer en el
        // render por software, dejando la escena negra al arrancar
        // (la «mancha negra» al abrir el programa).
        let scale = tier.render_scale();
        let rw = ((vw * scale) as u32).max(320);
        let rh = ((vh * scale) as u32).max(240);
        let key = (rw, rh, tier);
        if self.render_target.is_none() || self.render_target_key != key {
            let rt = render_target(rw, rh);
            rt.texture.set_filter(FilterMode::Linear);
            self.render_target = Some(rt);
            self.render_target_key = key;
        }
        cam3d.render_target = self.render_target.clone();
        // Sin viewport: al renderizar a destino, macroquad usa el
        // tamano de la textura (si se fija, la escena se recorta).
        cam3d.viewport = None;
        set_camera(&cam3d);
        clear_background(crate::color::BACKGROUND);

        scene::draw_lab(sim.observe, &sim.params, tier.ambient_detail());

        // Superficie de I(x): se reconstruye solo si cambió el estado físico.
        let mut key = WaveKey::of(sim);
        key.quality = tier;
        if self.wave_key.as_ref() != Some(&key) {
            self.wave_meshes = Some(build_wave_meshes(sim, tier.wave_resolution()));
            self.beam_meshes = Some(build_beams(&sim.params));
            self.wave_key = Some(key);
        }
        if self.layers.beams {
            if let Some(meshes) = &self.beam_meshes {
                for mesh in meshes {
                    draw_mesh(mesh);
                }
            }
        }
        if self.layers.wave {
            if let Some(meshes) = &self.wave_meshes {
                for mesh in meshes {
                    draw_mesh(mesh);
                }
            }
        }

        self.impacts.draw(sim, tier.impact_draw_cap(), tier.halo());
        let time = get_time() as f32;
        if self.layers.wave {
            if tier.contours() {
                draw_wave_contours(sim);
            }
            draw_wavefronts(sim, time);
            draw_profile_glow(sim);
        }
        if self.layers.particles {
            self.particles
                .update_and_draw(dt, sim.params.wavelength_m, tier);
        }

        set_default_camera();
        if let Some(rt) = &self.render_target {
            draw_texture_ex(
                &rt.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(vw, vh)),
                    flip_y: true,
                    ..Default::default()
                },
            );
        }
        if self.presentation {
            crate::ui::text(
                "Doble rendija — modo presentación  (H para salir)",
                20.0,
                34.0,
                17.0,
                crate::color::TEXT,
                crate::ui::Weight::SemiBold,
            );
            return ControlOutcome::default();
        }
        if self.layers.annotations {
            annotations::draw_annotations(&self.camera, vw);
        }
        draw_hud(sim, hud, self.layers.chart, vw);
        let out = self.panel.interact(
            sim,
            rate,
            paused,
            &mut self.layers,
            &mut self.camera,
            &mut self.cinematic,
            &mut self.presentation,
            &mut self.quality_mode,
            tier,
        );
        if out.reset_visuals {
            self.reset_visuals();
        }
        out
    }
}
