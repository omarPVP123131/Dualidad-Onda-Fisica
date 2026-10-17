//! Estado de la aplicación: simulación + preferencias de sesión.

use dualidad_core::{Simulation, SplitMix64};

/// Todo lo que vive entre frames y no pertenece ni al núcleo ni al motor.
pub struct AppState {
    pub sim: Simulation,
    pub rng: SplitMix64,
    /// Partículas generadas por frame (como en el prototipo).
    pub rate: u32,
    pub paused: bool,
}

impl AppState {
    pub fn new(seed: u64) -> Self {
        Self {
            sim: Simulation::default(),
            rng: SplitMix64::new(seed),
            rate: 20,
            paused: false,
        }
    }

    /// Ajuste de ritmo: duplicar con tope, dividir con suelo 1.
    pub fn rate_up(&mut self) {
        self.rate = (self.rate * 2).min(2_000);
    }

    pub fn rate_down(&mut self) {
        self.rate = (self.rate / 2).max(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_ritmo_satura_en_ambos_extremos() {
        let mut s = AppState::new(1);
        for _ in 0..20 {
            s.rate_up();
        }
        assert_eq!(s.rate, 2_000);
        for _ in 0..20 {
            s.rate_down();
        }
        assert_eq!(s.rate, 1);
    }
}
