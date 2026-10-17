//! Generador pseudoaleatorio propio y determinista (SplitMix64).
//!
//! Vivir en el núcleo (sin depender del azar de macroquad) hace la
//! simulación reproducible: misma semilla ⇒ misma secuencia de impactos,
//! lo que permite tests estadísticos estables.

/// Fuente de azar uniforme en `[0, 1)`.
pub trait RandomSource {
    fn next_f64(&mut self) -> f64;
}

/// SplitMix64: rápido, portable y de calidad suficiente para el muestreo
/// por rechazo de este simulador (no es criptográfico).
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

impl RandomSource for SplitMix64 {
    fn next_f64(&mut self) -> f64 {
        // 53 bits superiores → [0, 1) con precisión de f64.
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misma_semilla_misma_secuencia() {
        let mut a = SplitMix64::new(42);
        let mut b = SplitMix64::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_f64(), b.next_f64());
        }
    }

    #[test]
    fn valores_en_rango_y_media_razonable() {
        let mut rng = SplitMix64::new(0xD1B5_4A32_D192_ED03);
        let n = 20_000;
        let mut suma = 0.0;
        for _ in 0..n {
            let v = rng.next_f64();
            assert!((0.0..1.0).contains(&v));
            suma += v;
        }
        let media = suma / n as f64;
        assert!((media - 0.5).abs() < 0.02, "media={media}");
    }
}
