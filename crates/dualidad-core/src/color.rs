//! Conversión de longitud de onda visible (380–750 nm) a color RGB.
//!
//! Aproximación espectral estándar por tramos. Es lógica pura: devuelve
//! componentes en `[0, 1]` y cada frontal las convierte a su tipo color.

/// Color RGB con componentes en `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
}

/// Color aproximado del espectro visible para `wavelength_m` (metros).
pub fn wavelength_to_rgb(wavelength_m: f64) -> Rgb {
    let nm = wavelength_m * 1e9;
    // Fuera del rango visible (p. ej. el electrón de de Broglie,
    // 0.123 nm) se satura al extremo más cercano: sin esto, la primera
    // rama crecía sin límite ((440 − 0.123)/60 ≈ 7.3) y todo se teñía
    // de rojo saturado.
    let nm = nm.clamp(380.0, 750.0);
    let (r, g, b) = match nm {
        n if n < 440.0 => ((440.0 - n) / 60.0, 0.0, 1.0),
        n if n < 490.0 => (0.0, (n - 440.0) / 50.0, 1.0),
        n if n < 510.0 => (0.0, 1.0, (510.0 - n) / 20.0),
        n if n < 580.0 => ((n - 510.0) / 70.0, 1.0, 0.0),
        n if n < 645.0 => (1.0, (645.0 - n) / 65.0, 0.0),
        _ => (1.0, 0.0, 0.0),
    };
    Rgb {
        r: r as f32,
        g: g as f32,
        b: b as f32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuera_del_visible_satura_sin_desbordar() {
        // Electrón de 100 eV (0.1226 nm) y un infrarrojo lejano: los
        // canales deben quedarse en [0, 1] (antes r llegaba a ≈7.3).
        for wl in [0.1226e-9, 3_000e-9] {
            let c = wavelength_to_rgb(wl);
            assert!((0.0..=1.0).contains(&c.r), "r={}", c.r);
            assert!((0.0..=1.0).contains(&c.g), "g={}", c.g);
            assert!((0.0..=1.0).contains(&c.b), "b={}", c.b);
        }
        let electron = wavelength_to_rgb(0.1226e-9);
        let violeta = wavelength_to_rgb(380e-9);
        assert!(
            (electron.r - violeta.r).abs() < 0.01
                && electron.g == violeta.g
                && electron.b == violeta.b,
            "fuera del visible debe saturar al violeta: {electron:?}"
        );
    }

    #[test]
    fn extremos_del_espectro() {
        let violeta = wavelength_to_rgb(380e-9);
        assert_eq!(violeta.b, 1.0);
        assert_eq!(violeta.g, 0.0);
        let rojo = wavelength_to_rgb(750e-9);
        assert_eq!(
            rojo,
            Rgb {
                r: 1.0,
                g: 0.0,
                b: 0.0
            }
        );
    }

    #[test]
    fn el_verde_550_es_verde() {
        let c = wavelength_to_rgb(550e-9);
        assert_eq!(c.g, 1.0);
    }
}
