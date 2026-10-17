//! Parámetros físicos del experimento y sus rangos válidos.

/// Parámetros del experimento de la doble rendija (unidades SI).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationParams {
    /// Longitud de onda λ (m).
    pub wavelength_m: f64,
    /// Separación entre centros de rendijas `d` (m).
    pub slit_separation_m: f64,
    /// Ancho de cada rendija `a` (m).
    pub slit_width_m: f64,
    /// Distancia rendijas → pantalla `L` (m).
    pub screen_distance_m: f64,
    /// Semiancho de la pantalla (m). La pantalla cubre `[-half, +half]`.
    pub screen_half_width_m: f64,
    /// Número de bins del histograma.
    pub bins: usize,
    /// Número de rendijas (1 = difracción pura, 2 = doble rendija…).
    pub slit_count: usize,
}

impl Default for SimulationParams {
    fn default() -> Self {
        Self {
            wavelength_m: 550e-9,
            slit_separation_m: 60e-6,
            slit_width_m: 10e-6,
            screen_distance_m: 1.0,
            screen_half_width_m: 0.02,
            bins: 500,
            slit_count: 2,
        }
    }
}

impl SimulationParams {
    // Rangos aceptados por la interfaz (idénticos al prototipo 2D).
    pub const WAVELENGTH_RANGE_NM: (f64, f64) = (380.0, 750.0);
    pub const SEPARATION_RANGE_UM: (f64, f64) = (0.5, 200.0);
    pub const WIDTH_RANGE_UM: (f64, f64) = (0.1, 40.0);

    pub fn wavelength_nm(&self) -> f64 {
        self.wavelength_m * 1e9
    }

    pub fn separation_um(&self) -> f64 {
        self.slit_separation_m * 1e6
    }

    pub fn width_um(&self) -> f64 {
        self.slit_width_m * 1e6
    }

    /// Ajusta λ en `delta_nm`, saturando en el rango visible permitido.
    pub fn adjust_wavelength_nm(&mut self, delta_nm: f64) {
        let (min, max) = Self::WAVELENGTH_RANGE_NM;
        self.wavelength_m = (self.wavelength_nm() + delta_nm).clamp(min, max) * 1e-9;
    }

    /// Ajusta `d` en `delta_um`, saturando en su rango permitido.
    pub fn adjust_separation_um(&mut self, delta_um: f64) {
        let (min, max) = Self::SEPARATION_RANGE_UM;
        self.slit_separation_m = (self.separation_um() + delta_um).clamp(min, max) * 1e-6;
    }

    /// Ajusta `a` en `delta_um`, saturando en su rango permitido.
    pub fn adjust_width_um(&mut self, delta_um: f64) {
        let (min, max) = Self::WIDTH_RANGE_UM;
        self.slit_width_m = (self.width_um() + delta_um).clamp(min, max) * 1e-6;
    }

    /// Convierte una posición `x` (m) a coordenada normalizada `u ∈ [0, 1]`.
    pub fn to_normalized(&self, x_m: f64) -> f64 {
        (x_m + self.screen_half_width_m) / (2.0 * self.screen_half_width_m)
    }

    /// Convierte una coordenada normalizada `u ∈ [0, 1]` a posición `x` (m).
    pub fn to_position(&self, u: f64) -> f64 {
        -self.screen_half_width_m + u * 2.0 * self.screen_half_width_m
    }
}

impl SimulationParams {
    pub const DISTANCE_RANGE_M: (f64, f64) = (0.2, 2.5);
    pub const SLIT_COUNT_RANGE: (usize, usize) = (1, 8);

    pub fn set_wavelength_nm(&mut self, nm: f64) {
        let (min, max) = Self::WAVELENGTH_RANGE_NM;
        self.wavelength_m = nm.clamp(min, max) * 1e-9;
    }

    pub fn set_separation_um(&mut self, um: f64) {
        let (min, max) = Self::SEPARATION_RANGE_UM;
        self.slit_separation_m = um.clamp(min, max) * 1e-6;
    }

    pub fn set_width_um(&mut self, um: f64) {
        let (min, max) = Self::WIDTH_RANGE_UM;
        self.slit_width_m = um.clamp(min, max) * 1e-6;
    }

    pub fn set_distance_m(&mut self, m: f64) {
        let (min, max) = Self::DISTANCE_RANGE_M;
        self.screen_distance_m = m.clamp(min, max);
    }

    pub fn set_slit_count(&mut self, n: usize) {
        let (min, max) = Self::SLIT_COUNT_RANGE;
        self.slit_count = n.clamp(min, max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajustes_saturan_en_los_limites() {
        let mut p = SimulationParams::default();
        p.adjust_wavelength_nm(10_000.0);
        assert_eq!(p.wavelength_nm(), 750.0);
        p.adjust_wavelength_nm(-10_000.0);
        assert_eq!(p.wavelength_nm(), 380.0);
        p.adjust_separation_um(-10_000.0);
        assert_eq!(p.separation_um(), 0.5);
        p.adjust_width_um(10_000.0);
        assert_eq!(p.width_um(), 40.0);
    }

    #[test]
    fn conversion_posicion_es_inversible() {
        let p = SimulationParams::default();
        for u in [0.0, 0.25, 0.5, 1.0] {
            assert!((p.to_normalized(p.to_position(u)) - u).abs() < 1e-12);
        }
        assert_eq!(p.to_position(0.5), 0.0);
    }
}
