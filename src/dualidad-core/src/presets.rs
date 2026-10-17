//! Experimentos preconfigurados (fotones visibles y un electrón).
//!
//! Un preset fija el experimento completo, no solo λ: con un electrón
//! de 100 eV (λ ≈ 0.123 nm, de Broglie) las rendijas y la pantalla
//! deben escalar, o las franjas quedarían fuera de la ventana visible.

use crate::params::SimulationParams;

/// Constantes (SI exactas / CODATA).
const H: f64 = 6.626_070_15e-34; // Planck
const M_E: f64 = 9.109_383_7e-31; // masa del electrón
const E_CHARGE: f64 = 1.602_176_634e-19; // carga elemental
const C: f64 = 299_792_458.0; // velocidad de la luz

/// λ de de Broglie (relativista) para un electrón acelerado con `ev` voltios.
pub fn electron_wavelength_m(ev: f64) -> f64 {
    let energy_j = ev * E_CHARGE;
    let momentum = ((energy_j + M_E * C * C).powi(2) - (M_E * C * C).powi(2)).sqrt() / C;
    H / momentum
}

/// Un experimento listo para cargar.
#[derive(Debug, Clone)]
pub struct Preset {
    pub name: &'static str,
    pub detail: &'static str,
    pub params: SimulationParams,
}

fn photon(name: &'static str, detail: &'static str, nm: f64) -> Preset {
    let mut params = SimulationParams::default();
    params.set_wavelength_nm(nm);
    Preset {
        name,
        detail,
        params,
    }
}

/// Catálogo de presets de la interfaz.
pub fn catalog() -> Vec<Preset> {
    let electron = {
        let mut params = SimulationParams {
            wavelength_m: electron_wavelength_m(100.0),
            ..Default::default()
        };
        params.set_separation_um(2.0);
        params.set_width_um(0.5);
        params.set_distance_m(1.0);
        params.screen_half_width_m = 0.002; // ventana de ±2 mm
        params.slit_count = 2;
        params
    };
    vec![
        photon("Fotón verde", "550 nm · visible", 550.0),
        photon("Fotón azul", "450 nm · visible", 450.0),
        photon("Fotón rojo", "650 nm · visible", 650.0),
        Preset {
            name: "Electrón 100 eV",
            detail: "de Broglie · 0.123 nm",
            params: electron,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn electron_de_100_ev() {
        let lambda = electron_wavelength_m(100.0);
        // Valor conocido: ≈ 0.1226 nm.
        assert!((lambda - 0.1226e-9).abs() < 0.002e-9, "λ={lambda:e}");
    }

    #[test]
    fn el_catalogo_es_utilizable() {
        for preset in catalog() {
            assert!(preset.params.wavelength_m > 0.0);
            assert!(preset.params.slit_count >= 1);
        }
    }
}
