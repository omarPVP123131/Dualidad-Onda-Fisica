//! Calidad gráfica adaptativa: el simulador debe ir fluido también en
//! máquinas sin GPU (render por software / Mesa), típicas en escuelas.
//!
//! Tres niveles. En modo automático el renderer mide los fps y baja de
//! nivel si hace falta (y vuelve a subir solo con margen de sobra).
//! Bajar de nivel **nunca** toca la física ni los datos: solo cambia
//! cuánto se dibuja de ellos.

/// Nivel efectivo de detalle visual.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QualityTier {
    #[default]
    Alta,
    Equilibrada,
    Ligera,
}

/// Lo que elige el usuario en el panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QualityMode {
    #[default]
    Auto,
    Alta,
    Ligera,
}

impl QualityTier {
    pub fn label(self) -> &'static str {
        match self {
            QualityTier::Alta => "Alta",
            QualityTier::Equilibrada => "Equilibrada",
            QualityTier::Ligera => "Ligera",
        }
    }

    /// Resolución (columnas, filas) de la superficie de onda.
    pub fn wave_resolution(self) -> (usize, usize) {
        match self {
            QualityTier::Alta => (180, 56),
            QualityTier::Equilibrada => (140, 44),
            QualityTier::Ligera => (105, 32),
        }
    }

    /// Cuántos impactos se dibujan como máximo (el histograma y la
    /// cuenta total siempre usan **todos** los datos reales).
    pub fn impact_draw_cap(self) -> usize {
        match self {
            QualityTier::Alta => 40_000,
            QualityTier::Equilibrada => 14_000,
            QualityTier::Ligera => 6_000,
        }
    }

    pub fn halo(self) -> bool {
        !matches!(self, QualityTier::Ligera)
    }

    pub fn particles_cap(self) -> usize {
        match self {
            QualityTier::Alta => 240,
            QualityTier::Equilibrada => 120,
            QualityTier::Ligera => 55,
        }
    }

    /// Escala de renderizado de la escena 3D (la UI siempre va a
    /// resolución completa). Es la palanca que más ahorra sin GPU:
    /// la mitad de escala ≈ la cuarta parte de píxeles que rellenar.
    pub fn render_scale(self) -> f32 {
        match self {
            QualityTier::Alta => 1.0,
            QualityTier::Equilibrada => 0.75,
            QualityTier::Ligera => 0.55,
        }
    }

    /// Detalle ambiental (polvo, resplandor, escaneo de pantalla).
    pub fn ambient_detail(self) -> bool {
        matches!(self, QualityTier::Alta)
    }

    pub fn contours(self) -> bool {
        !matches!(self, QualityTier::Ligera)
    }

    fn lower(self) -> Self {
        match self {
            QualityTier::Alta => QualityTier::Equilibrada,
            _ => QualityTier::Ligera,
        }
    }

    fn raise(self) -> Self {
        match self {
            QualityTier::Ligera => QualityTier::Equilibrada,
            _ => QualityTier::Alta,
        }
    }
}

/// Autoajuste con histéresis: baja rápido si falta fluidez, sube lento
/// y solo con margen, para no oscilar entre niveles.
#[derive(Debug, Clone)]
pub struct AutoQuality {
    pub tier: QualityTier,
    frames: u32,
    good_seconds: f32,
}

impl Default for AutoQuality {
    fn default() -> Self {
        Self {
            tier: QualityTier::Alta,
            frames: 0,
            good_seconds: 0.0,
        }
    }
}

impl AutoQuality {
    /// Alimentar una vez por frame con el `dt` real.
    pub fn update(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }
        self.frames += 1;
        // Ignorar el arranque (carga de mallas) antes de juzgar.
        if self.frames < 90 {
            return;
        }
        let fps = 1.0 / dt;
        if fps < 27.0 && self.tier != QualityTier::Ligera {
            self.tier = self.tier.lower();
            self.frames = 0;
            self.good_seconds = 0.0;
        } else if fps > 57.0 && self.tier != QualityTier::Alta {
            self.good_seconds += dt;
            if self.good_seconds > 8.0 {
                self.tier = self.tier.raise();
                self.good_seconds = 0.0;
            }
        } else {
            self.good_seconds = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bajar_de_nivel_es_monotono() {
        assert_eq!(QualityTier::Alta.lower(), QualityTier::Equilibrada);
        assert_eq!(QualityTier::Equilibrada.lower(), QualityTier::Ligera);
        assert_eq!(QualityTier::Ligera.lower(), QualityTier::Ligera);
    }

    #[test]
    fn con_frames_lentos_baja_a_ligera() {
        let mut q = AutoQuality::default();
        // Arranque ignorado + dos ventanas de juicio a ~15 fps.
        for _ in 0..400 {
            q.update(1.0 / 15.0);
        }
        assert_eq!(q.tier, QualityTier::Ligera);
    }
}
