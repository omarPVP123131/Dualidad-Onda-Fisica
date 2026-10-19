//! Estado de la simulación: parámetros, impactos y muestreo por rechazo.

use crate::histogram::Histogram;
use crate::params::SimulationParams;
use crate::physics;
use crate::rng::RandomSource;

/// Un impacto individual sobre la pantalla.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Impact {
    /// Posición física sobre la pantalla (m).
    pub x_m: f64,
    /// Posición normalizada `u ∈ [0, 1]` (para histograma y dibujo).
    pub u: f64,
    /// Coordenada vertical asignada al dibujarlo en la pantalla `v ∈ [0, 1]`.
    /// No tiene significado físico: reparte los puntos en el alto.
    pub v: f64,
}

/// Simulación completa y frontal-agnóstica.
#[derive(Debug, Clone)]
pub struct Simulation {
    pub params: SimulationParams,
    /// Detector *which-path* activo.
    pub observe: bool,
    histogram: Histogram,
    impacts: Vec<Impact>,
    /// Máximo de impactos conservados para dibujar (la cuenta no se detiene).
    /// Al alcanzarlo se conservan los **más recientes**: la ventana se
    /// desliza y los impactos nuevos nunca dejan de almacenarse.
    pub max_stored_impacts: usize,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new(SimulationParams::default())
    }
}

impl Simulation {
    /// Tope de impactos conservados para dibujar (la cuenta no se detiene).
    pub const DEFAULT_MAX_STORED: usize = 40_000;
    /// Bloque de recorte del almacén: al llegar al tope se descartan los
    /// impactos más antiguos de golpe, no uno a uno (amortiza el coste).
    pub const TRIM_CHUNK: usize = 1_024;
    /// Cota de intentos por impacto: evita bucles infinitos si `I(x)`
    /// fuera casi cero en casi toda la pantalla.
    const MAX_ATTEMPTS: usize = 100_000;

    pub fn new(params: SimulationParams) -> Self {
        Self {
            histogram: Histogram::new(params.bins),
            params,
            observe: false,
            impacts: Vec::new(),
            max_stored_impacts: Self::DEFAULT_MAX_STORED,
        }
    }

    pub fn reset(&mut self) {
        self.histogram.reset();
        self.impacts.clear();
    }

    pub fn histogram(&self) -> &Histogram {
        &self.histogram
    }

    pub fn impacts(&self) -> &[Impact] {
        &self.impacts
    }

    pub fn total(&self) -> u64 {
        self.histogram.total()
    }

    /// Dispara UNA partícula.
    ///
    /// Muestreo por rechazo sobre `I(x)`: se propone un `x` uniforme y se
    /// acepta con probabilidad `I(x)`. La partícula no «sabe» dónde caerá;
    /// el patrón de interferencia emerge solo al acumular impactos.
    ///
    /// Devuelve el impacto generado, o `None` si se agotaron los intentos
    /// (no ocurre con parámetros físicos razonables).
    pub fn shoot_one(&mut self, rng: &mut impl RandomSource) -> Option<Impact> {
        let half = self.params.screen_half_width_m;
        for _ in 0..Self::MAX_ATTEMPTS {
            let x = -half + 2.0 * half * rng.next_f64();
            if rng.next_f64() < physics::intensity(&self.params, x, self.observe) {
                let impact = Impact {
                    x_m: x,
                    u: self.params.to_normalized(x),
                    v: rng.next_f64(),
                };
                self.histogram.record(impact.u);
                // Ventana deslizante: al llenarse se sueltan los impactos
                // más antiguos (ya contados en el histograma) para que los
                // recientes siempre entren. Congelarse aquí dejaba a la
                // pantalla y a las partículas sin datos nuevos.
                self.impacts.push(impact);
                if self.impacts.len() >= self.max_stored_impacts + Self::TRIM_CHUNK {
                    let recorte = self.impacts.len() - self.max_stored_impacts;
                    self.impacts.drain(..recorte);
                }
                return Some(impact);
            }
        }
        None
    }

    /// Dispara `n` partículas y devuelve cuántas se generaron.
    pub fn shoot_many(&mut self, rng: &mut impl RandomSource, n: u32) -> u32 {
        let mut hechas = 0;
        for _ in 0..n {
            if self.shoot_one(rng).is_some() {
                hechas += 1;
            }
        }
        hechas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::SplitMix64;

    #[test]
    fn disparar_acumula_histograma_e_impactos() {
        let mut sim = Simulation::default();
        let mut rng = SplitMix64::new(7);
        let n = sim.shoot_many(&mut rng, 200);
        assert_eq!(n, 200);
        assert_eq!(sim.total(), 200);
        assert_eq!(sim.impacts().len(), 200);
        assert!(sim.impacts().iter().all(|i| (0.0..=1.0).contains(&i.u)));
    }

    #[test]
    fn el_centro_recibe_mas_que_los_bordes() {
        // Propiedad central del fenómeno: el máximo central domina.
        let mut sim = Simulation::default();
        let mut rng = SplitMix64::new(1234);
        sim.shoot_many(&mut rng, 8_000);
        let bins = sim.histogram().bins();
        let centro: u32 = bins[bins.len() / 2 - 5..=bins.len() / 2 + 5].iter().sum();
        let borde: u32 = bins[..11].iter().sum();
        assert!(centro * 2 > borde * 3, "centro={centro} borde={borde}");
    }

    #[test]
    fn con_detector_el_centro_sigue_dominando_pero_sin_franjas() {
        // No testeamos "cada franja" aquí: la ausencia de cos² ya está
        // cubierta en physics; esto verifica el camino de simulación.
        let mut sim = Simulation {
            observe: true,
            ..Default::default()
        };
        let mut rng = SplitMix64::new(99);
        sim.shoot_many(&mut rng, 2_000);
        assert_eq!(sim.total(), 2_000);
    }

    #[test]
    fn reset_limpia_todo() {
        let mut sim = Simulation::default();
        let mut rng = SplitMix64::new(1);
        sim.shoot_many(&mut rng, 50);
        sim.reset();
        assert_eq!(sim.total(), 0);
        assert!(sim.impacts().is_empty());
    }

    #[test]
    fn la_ventana_sigue_guardando_los_impactos_recientes() {
        // Con el almacén «saturado» (mucho más disparos que el tope), los
        // impactos nuevos deben seguir entrando: si no, la pantalla y las
        // partículas en vuelo se quedan sin datos (ADR-0005).
        let mut sim = Simulation {
            max_stored_impacts: 500,
            ..Default::default()
        };
        let mut rng = SplitMix64::new(3);
        sim.shoot_many(&mut rng, 3_000);
        assert_eq!(sim.total(), 3_000);
        let tope = 500 + Simulation::TRIM_CHUNK;
        let len = sim.impacts().len();
        assert!(len > 0 && len <= tope, "len={len} tope={tope}");

        // El impacto recién disparado siempre se guarda.
        let siguiente = sim.shoot_one(&mut rng).expect("siempre hay impacto");
        assert_eq!(sim.impacts().last(), Some(&siguiente));
        assert_eq!(sim.total(), 3_001);

        // Y la ventana se desliza: los más antiguos salen al llenarse.
        let primero = sim.impacts()[0];
        sim.shoot_many(&mut rng, Simulation::TRIM_CHUNK as u32 + 1);
        assert_ne!(sim.impacts()[0], primero, "los antiguos deben salir");
        assert!(sim.impacts().len() <= tope);
        assert_eq!(sim.total(), 3_001 + Simulation::TRIM_CHUNK as u64 + 1);
    }
}
