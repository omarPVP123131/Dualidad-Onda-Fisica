//! Modelo físico: intensidad de Fraunhofer de la doble rendija.

use crate::params::SimulationParams;
use std::f64::consts::PI;

/// `sinc(t) = sin(t)/t`, con la extensión continua `sinc(0) = 1`.
pub fn sinc(t: f64) -> f64 {
    if t.abs() < 1e-12 {
        1.0
    } else {
        t.sin() / t
    }
}

/// Factor de interferencia de `n` rendijas equiespaciadas, con
/// `delta = π·d·x/(λ·L)` como en el resto del módulo:
/// `[ sin(n·delta) / (n·sin(delta)) ]²`. Para n=2 se reduce a `cos²(delta)`;
/// para n=1 vale 1 (no hay interferencia).
pub fn interference_factor(n: usize, delta: f64) -> f64 {
    if n <= 1 {
        return 1.0;
    }
    let denom = delta.sin();
    if denom.abs() < 1e-9 {
        return 1.0; // límite en los máximos principales
    }
    ((n as f64 * delta).sin() / (n as f64 * denom)).powi(2)
}

/// Intensidad relativa `I(x) ∈ [0, 1]` en la pantalla.
///
/// ```text
/// I(x) = sinc²(β) · F_N(δ)
/// β = π·a·x / (λ·L)   (difracción de cada rendija)
/// δ = π·d·x / (λ·L)   (interferencia entre rendijas)
/// ```
///
/// Con el detector *which-path* activo (`observe = true`) desaparece el
/// término de interferencia y queda solo la envolvente `sinc²(β)`.
pub fn intensity(params: &SimulationParams, x_m: f64, observe: bool) -> f64 {
    let k = PI * x_m / (params.wavelength_m * params.screen_distance_m);
    let envelope = sinc(k * params.slit_width_m).powi(2);
    if observe {
        envelope
    } else {
        envelope * interference_factor(params.slit_count, k * params.slit_separation_m)
    }
}

/// Curva teórica sobre los centros de los `bins` del histograma.
///
/// Útil para comparar el histograma acumulado con lo esperado sin
/// recalcular la fórmula desde la capa gráfica.
pub fn theoretical_curve(params: &SimulationParams, observe: bool) -> Vec<f64> {
    (0..params.bins)
        .map(|i| {
            let u = (i as f64 + 0.5) / params.bins as f64;
            intensity(params, params.to_position(u), observe)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> SimulationParams {
        SimulationParams::default()
    }

    #[test]
    fn el_centro_es_el_maximo_global() {
        let p = params();
        assert_eq!(intensity(&p, 0.0, false), 1.0);
        for i in -50..=50 {
            let x = i as f64 * 0.0004;
            assert!(intensity(&p, x, false) <= 1.0 + 1e-12, "x={x}");
        }
    }

    #[test]
    fn la_intensidad_es_simetrica() {
        let p = params();
        for i in 1..40 {
            let x = i as f64 * 0.0005;
            assert!((intensity(&p, x, false) - intensity(&p, -x, false)).abs() < 1e-12);
        }
    }

    #[test]
    fn con_detector_no_hay_franjas_de_interferencia() {
        let p = params();
        assert_eq!(intensity(&p, 0.0, true), 1.0);
        // La envolvente es monótona decreciente al alejarse del centro
        // dentro de su lóbulo central: sin cos² no hay máximos secundarios
        // creados por interferencia a corta distancia.
        let mut anterior = f64::INFINITY;
        for i in 1..12 {
            let v = intensity(&p, i as f64 * 0.0002, true);
            assert!(v <= anterior + 1e-12);
            anterior = v;
        }
    }

    #[test]
    fn la_intensidad_nunca_es_negativa() {
        let p = params();
        for i in -100..=100 {
            let x = i as f64 * 0.0002;
            let v = intensity(&p, x, false);
            assert!((0.0..=1.0 + 1e-12).contains(&v), "I({x}) = {v}");
        }
    }

    #[test]
    fn sinc_en_cero_es_uno() {
        assert_eq!(sinc(0.0), 1.0);
        assert!((sinc(1e-15) - 1.0).abs() < 1e-12);
    }
}

#[cfg(test)]
mod n_slit_tests {
    use super::*;

    #[test]
    fn una_rendija_es_solo_difraccion() {
        let p = SimulationParams {
            slit_count: 1,
            ..Default::default()
        };
        for i in -20..=20 {
            let x = i as f64 * 0.001;
            let esperado =
                sinc(PI * x * p.slit_width_m / (p.wavelength_m * p.screen_distance_m)).powi(2);
            assert!((intensity(&p, x, false) - esperado).abs() < 1e-12);
        }
    }

    #[test]
    fn dos_rendijas_equivalen_a_cos_al_cuadrado() {
        for d in [0.0, 0.4, 1.3, 3.1] {
            assert!(
                (interference_factor(2, d) - (d).cos().powi(2)).abs() < 1e-9,
                "delta={d}"
            );
        }
    }
}
