//! Histograma de impactos sobre la pantalla.

use crate::params::SimulationParams;

/// Conteos por intervalo de posición. El bin `i` cubre el tramo
/// `i / bins` de la pantalla normalizada.
#[derive(Debug, Clone)]
pub struct Histogram {
    bins: Vec<u32>,
    total: u64,
}

impl Histogram {
    pub fn new(bins: usize) -> Self {
        Self {
            bins: vec![0; bins.max(1)],
            total: 0,
        }
    }

    /// Registra un impacto en la posición normalizada `u ∈ [0, 1]`.
    /// Valores fuera de rango saturan al primer/último bin.
    pub fn record(&mut self, u: f64) {
        let idx = (u.clamp(0.0, 1.0) * self.bins.len() as f64) as usize;
        let idx = idx.min(self.bins.len() - 1);
        self.bins[idx] += 1;
        self.total += 1;
    }

    pub fn reset(&mut self) {
        self.bins.iter_mut().for_each(|b| *b = 0);
        self.total = 0;
    }

    /// Redimensiona conservando el total en cero (para cambios de `bins`).
    pub fn resize(&mut self, bins: usize, params_bins: usize) {
        debug_assert_eq!(bins, params_bins);
        self.bins = vec![0; bins.max(1)];
        self.total = 0;
    }

    pub fn bins(&self) -> &[u32] {
        &self.bins
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn max_count(&self) -> u32 {
        self.bins.iter().copied().max().unwrap_or(0)
    }

    /// Conteo esperado por bin según la teoría, dado el total observado.
    ///
    /// Normaliza la curva teórica para que sume `total`: así el histograma
    /// y la teoría se comparan en la misma escala.
    pub fn expected_counts(&self, params: &SimulationParams, observe: bool) -> Vec<f64> {
        let curve = crate::physics::theoretical_curve(params, observe);
        let sum: f64 = curve.iter().sum();
        if sum <= 0.0 || self.total == 0 {
            return vec![0.0; curve.len()];
        }
        curve.iter().map(|v| self.total as f64 * v / sum).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registrar_y_reiniciar() {
        let mut h = Histogram::new(10);
        h.record(0.0);
        h.record(0.5);
        h.record(1.0);
        h.record(7.0); // fuera de rango: satura al último bin
        assert_eq!(h.total(), 4);
        assert_eq!(h.bins()[9], 2);
        h.reset();
        assert_eq!(h.total(), 0);
        assert!(h.bins().iter().all(|&b| b == 0));
    }

    #[test]
    fn lo_esperado_suma_el_total() {
        let params = SimulationParams::default();
        let mut h = Histogram::new(params.bins);
        for _ in 0..100 {
            h.record(0.5);
        }
        let esperado: f64 = h.expected_counts(&params, false).iter().sum();
        assert!((esperado - 100.0).abs() < 1e-6);
    }
}
