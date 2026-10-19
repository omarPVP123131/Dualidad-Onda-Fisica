//! Partículas en vuelo: recurso visual para seguir cada disparo.
//!
//! La trayectoria dibujada es **esquemática** (fuente → barrera →
//! rendija(s) → impacto en pantalla) y **siempre termina en el impacto**:
//! con el búfer lleno se dejan de crear vuelos nuevos, pero ninguno en
//! curso se recicla ni se corta a mitad de camino.
//!
//! Sin detector, cada partícula se dibuja **dividiéndose en todas las
//! rendijas**: las ramas nacen en la barrera y convergen en el mismo
//! impacto (la ilustración gráfica de la superposición). Con el detector
//! *which-path* activo se dibuja un único camino por la rendija más
//! cercana al destino, porque ahí sí hay una medida. La física real vive
//! en `dualidad-core`.
//!
//! Todo el vuelo (bloques y rastros) se agrupa en dos lotes por frame:
//! antes cada bloque y cada línea eran una subida de geometría al GPU.

use crate::batch::{LineBatch, MeshBatch, ViewBasis};
use crate::layout;
use crate::quality::QualityTier;
use crate::scene;
use dualidad_core::{Impact, SimulationParams};
use macroquad::prelude::*;

/// Tope de bloques dibujados a la vez (una rama cuenta uno).
const MAX_ACTIVE: usize = 240;
const MAX_SPAWN_PER_FRAME: usize = 8;
/// Máximo de rendijas del experimento (`SLIT_COUNT_RANGE` del núcleo).
const MAX_BRANCHES: usize = 8;
/// Duración del vuelo completo (segundos).
const DURATION: f32 = 0.85;
/// Fracción del vuelo en la que la cabeza alcanza la barrera y se divide.
const T_SPLIT: f32 = 0.42;
/// Fracción del vuelo en la que cada rama atraviesa su rendija.
const T_SLIT: f32 = 0.60;

/// Una rama de la trayectoria tras la barrera.
#[derive(Debug, Clone, Copy)]
struct Branch {
    /// Centro de la rendija por la que pasa esta rama.
    via: Vec3,
    /// Posición dibujada en el frame anterior (rastro).
    prev: Option<Vec3>,
}

/// Un disparo en vuelo: cabeza (fuente → barrera) y ramas (→ impacto).
#[derive(Debug, Clone, Copy)]
struct Flying {
    age: f32,
    /// Punto de la barrera donde la cabeza se divide.
    pivot: Vec3,
    /// Impacto en pantalla: destino común de todas las ramas.
    target: Vec3,
    branches: [Branch; MAX_BRANCHES],
    n_branches: usize,
    /// Rastro de la cabeza durante el primer tramo.
    head_prev: Option<Vec3>,
}

impl Flying {
    /// Bloques que este vuelo ocupa del presupuesto de dibujo.
    fn blobs(&self) -> usize {
        self.n_branches.max(1)
    }
}

/// Posición de la cabeza en el tramo 1 (`u` ∈ [0, 1] dentro del tramo).
fn head_pos(source: Vec3, pivot: Vec3, u: f32) -> Vec3 {
    source.lerp(pivot, u.clamp(0.0, 1.0))
}

/// Posición de una rama con `t` ∈ [0, 1] del vuelo completo:
/// barrera → rendija (`via`) y rendija → impacto (`target`).
fn branch_pos(pivot: Vec3, via: Vec3, target: Vec3, t: f32) -> Vec3 {
    if t < T_SLIT {
        let u = (t - T_SPLIT) / (T_SLIT - T_SPLIT);
        pivot.lerp(via, u.clamp(0.0, 1.0))
    } else {
        let u = (t - T_SLIT) / (1.0 - T_SLIT);
        via.lerp(target, u.clamp(0.0, 1.0))
    }
}

/// Centro de rendija más cercano a `x` (artificio del camino medido).
fn nearest(centers: &[f32], x: f32) -> f32 {
    centers
        .iter()
        .copied()
        .min_by(|a, b| (x - a).abs().total_cmp(&(x - b).abs()))
        .unwrap_or(0.0)
}

/// Color con alfa atenuado (el alfa base del color ya trae su peso).
fn tint(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, color.a * alpha.clamp(0.0, 1.0))
}

/// Mitad del ancho del rastro (≈1 px en la vista por defecto).
const TRAIL_HALF: f32 = 0.006;

/// Acumula el bloque de una partícula: disco orientado a la cámara en
/// Alta (misma silueta que la esfera, mucho más barato), cubo en el resto.
fn push_blob(
    blobs: &mut MeshBatch,
    pos: Vec3,
    fade: f32,
    color: Color,
    discs: bool,
    view: ViewBasis,
) {
    if discs {
        blobs.push_disc(
            pos,
            view.right,
            view.up,
            0.042 * (0.35 + 0.65 * fade),
            color,
        );
    } else {
        let s = 0.075 * (0.35 + 0.65 * fade);
        blobs.push_cube(pos, vec3(s, s, s), color);
    }
}

#[derive(Default)]
pub struct FlightParticles {
    active: Vec<Flying>,
    /// Lotes reutilizados cada frame (sin asignar en el bucle de dibujo).
    blobs: MeshBatch,
    trails: LineBatch,
}

impl FlightParticles {
    /// Registra impactos nuevos como vuelos en curso (con tope).
    ///
    /// Con el búfer lleno **no** se descarta ningún vuelo a medias: se
    /// omiten los impactos sobrantes de este frame. Reciclar el vuelo
    /// más antiguo cortaba su trayectoria en pleno vuelo y parecía que
    /// las partículas «nunca llegaban».
    pub fn spawn(
        &mut self,
        params: &SimulationParams,
        impacts: &[Impact],
        cap: usize,
        observe: bool,
    ) {
        if impacts.is_empty() {
            return;
        }
        let centers = scene::slit_centers_x(params);
        if centers.is_empty() {
            return;
        }
        let cap = cap.min(MAX_ACTIVE);
        let step = (impacts.len() / MAX_SPAWN_PER_FRAME).max(1);
        let mut used: usize = self.active.iter().map(Flying::blobs).sum();
        for impact in impacts.iter().step_by(step).take(MAX_SPAWN_PER_FRAME) {
            let x = layout::u_to_world_x(impact.u);
            // Sin detector la partícula «explora» todas las rendijas y se
            // dibuja dividida; con detector hay un solo camino definido.
            let n_branches = if observe {
                1
            } else {
                centers.len().min(MAX_BRANCHES)
            };
            if used + n_branches > cap {
                break;
            }
            let mut branches = [Branch {
                via: vec3(0.0, layout::OPTICAL_AXIS_Y, layout::SLITS_Z),
                prev: None,
            }; MAX_BRANCHES];
            let mut sum_x = 0.0f32;
            for (k, branch) in branches.iter_mut().take(n_branches).enumerate() {
                let cx = if observe {
                    nearest(&centers, x)
                } else {
                    centers[k]
                };
                branch.via = vec3(cx, layout::OPTICAL_AXIS_Y, layout::SLITS_Z);
                sum_x += cx;
            }
            // Con una sola rama el «punto de división» es su propia rendija
            // (la cabeza entra directa). Con varias, el centro de la barrera.
            let pivot = if n_branches == 1 {
                branches[0].via
            } else {
                vec3(
                    sum_x / n_branches as f32,
                    layout::OPTICAL_AXIS_Y,
                    layout::SLITS_Z,
                )
            };
            used += n_branches;
            self.active.push(Flying {
                age: 0.0,
                pivot,
                target: vec3(x, layout::impact_y(impact.v), layout::SCREEN_Z - 0.18),
                branches,
                n_branches,
                head_prev: None,
            });
        }
    }

    /// Avanza la edad de los vuelos y retira los que ya impactaron.
    pub fn update(&mut self, dt: f32) {
        self.active.retain_mut(|p| {
            p.age += dt;
            p.age / DURATION < 1.0
        });
    }

    /// Dibuja los vuelos en curso agrupando la geometría en dos lotes
    /// (bloques y rastros): dos llamadas de dibujo en vez de cientos.
    pub fn draw(&mut self, wavelength_m: f64, tier: QualityTier, view: ViewBasis) {
        let color = crate::color::wave_color(wavelength_m, 1.0);
        let source = vec3(0.0, layout::OPTICAL_AXIS_Y, layout::SOURCE_Z);
        let trails = tier != QualityTier::Ligera;
        let discs = tier == QualityTier::Alta;
        self.blobs.clear();
        self.trails.begin(view.forward);
        for p in self.active.iter_mut() {
            let t = p.age / DURATION;
            // Aparece y se desvanece con suavidad: sin esto el bloque
            // «explotaba» en la fuente y se apagaba de golpe al llegar.
            let fade_in = (t / 0.07).min(1.0);
            let fade_out = ((1.0 - t) / 0.13).min(1.0);
            let fade = fade_in.min(fade_out).clamp(0.0, 1.0);
            if t < T_SPLIT {
                // Tramo 1: cabeza de la fuente a la barrera.
                let u = t / T_SPLIT;
                let pos = head_pos(source, p.pivot, u);
                // Se desvanece justo antes de dividirse en la barrera.
                let alpha = fade * (1.0 - ((u - 0.82) / 0.18).clamp(0.0, 1.0));
                if trails {
                    if let Some(prev) = p.head_prev {
                        self.trails
                            .segment(prev, pos, TRAIL_HALF, tint(color, 0.45 * alpha));
                    }
                }
                push_blob(&mut self.blobs, pos, alpha, tint(color, alpha), discs, view);
                p.head_prev = Some(pos);
            } else {
                // Tramo 2: barrera → rendija de cada rama.
                // Tramo 3: rendija → impacto (todas convergen en el mismo punto).
                let pivot = p.pivot;
                let target = p.target;
                let fork_in = ((t - T_SPLIT) / 0.08).min(1.0);
                for branch in p.branches.iter_mut().take(p.n_branches) {
                    let pos = branch_pos(pivot, branch.via, target, t);
                    let alpha = fade * fork_in;
                    if trails {
                        if let Some(prev) = branch.prev {
                            self.trails
                                .segment(prev, pos, TRAIL_HALF, tint(color, 0.45 * alpha));
                        }
                    }
                    push_blob(&mut self.blobs, pos, alpha, tint(color, alpha), discs, view);
                    branch.prev = Some(pos);
                }
            }
        }
        self.blobs.draw();
        self.trails.draw();
    }

    pub fn clear(&mut self) {
        self.active.clear();
    }

    /// Vuelos en curso (para pruebas y diagnóstico).
    pub fn active_count(&self) -> usize {
        self.active.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dualidad_core::SimulationParams;

    fn impactos(n: usize) -> Vec<Impact> {
        (0..n)
            .map(|i| Impact {
                x_m: 0.0,
                u: (i as f64 % 97.0) / 97.0,
                v: (i as f64 % 53.0) / 53.0,
            })
            .collect()
    }

    #[test]
    fn el_destino_cae_sobre_la_pantalla() {
        let params = SimulationParams::default();
        let mut p = FlightParticles::default();
        let mut extremos = impactos(2);
        extremos[0].v = 0.0;
        extremos[1].v = 1.0;
        p.spawn(&params, &extremos, 240, false);
        assert_eq!(p.active.len(), 2);
        let bottom = layout::OPTICAL_AXIS_Y - layout::SCREEN_HEIGHT / 2.0;
        let top = layout::OPTICAL_AXIS_Y + layout::SCREEN_HEIGHT / 2.0;
        for f in &p.active {
            assert!(
                f.target.y > bottom && f.target.y < top,
                "destino fuera de la pantalla: y={}",
                f.target.y
            );
        }
    }

    #[test]
    fn sin_detector_el_vuelo_se_divide_en_todas_las_rendijas() {
        let params = SimulationParams::default();
        let mut p = FlightParticles::default();
        p.spawn(&params, &impactos(1), 240, false);
        assert_eq!(p.active_count(), 1);
        let f = &p.active[0];
        let centros = scene::slit_centers_x(&params);
        assert_eq!(f.n_branches, centros.len());
        for (rama, &cx) in f.branches.iter().zip(centros.iter()) {
            assert!(
                (rama.via.x - cx).abs() < 1e-6,
                "la rama no pasa por su rendija"
            );
        }
        // La cabeza entra recta hasta el centro de la barrera antes de dividirse.
        assert!(f.pivot.x.abs() < 1e-6, "pivote descentrado: {}", f.pivot.x);
    }

    #[test]
    fn con_detector_solo_se_dibuja_un_camino() {
        let params = SimulationParams::default();
        let mut p = FlightParticles::default();
        p.spawn(&params, &impactos(1), 240, true);
        let f = &p.active[0];
        let centros = scene::slit_centers_x(&params);
        assert_eq!(f.n_branches, 1);
        // El impacto del primer `impacto` cae en x = -4: rendija izquierda.
        assert!((f.branches[0].via.x - centros[0]).abs() < 1e-6);
        assert!((f.pivot.x - f.branches[0].via.x).abs() < 1e-6);
    }

    #[test]
    fn la_rama_termina_en_el_impacto() {
        let pivot = vec3(1.0, 2.0, 3.0);
        let via = vec3(-1.0, 4.0, 0.0);
        let target = vec3(2.5, 1.0, 5.82);
        // Continuidad: la rama nace en el pivote y pasa por la rendija.
        assert!(branch_pos(pivot, via, target, T_SPLIT).distance(pivot) < 1e-5);
        assert!(branch_pos(pivot, via, target, T_SLIT).distance(via) < 1e-5);
        // Y termina exactamente en el punto de impacto.
        assert!(branch_pos(pivot, via, target, 1.0).distance(target) < 1e-5);
        // La cabeza llega al pivote cuando comienza la división.
        assert!(head_pos(vec3(0.0, 2.0, -5.5), pivot, 1.0).distance(pivot) < 1e-5);
    }

    #[test]
    fn el_tope_no_interrumpe_los_vuelos_en_curso() {
        let params = SimulationParams::default();
        let mut p = FlightParticles::default();
        let imps = impactos(2);
        // Con 2 ramas por vuelo, un tope de 2 deja entrar exactamente uno.
        p.spawn(&params, &imps, 2, false);
        assert_eq!(p.active_count(), 1);
        for _ in 0..10 {
            p.update(0.05);
            p.spawn(&params, &imps, 2, false);
        }
        assert_eq!(p.active_count(), 1, "el vuelo en curso no se recicla");
        assert!(
            (p.active[0].age - 0.5).abs() < 1e-6,
            "su edad avanza: no se reinicia (age={})",
            p.active[0].age
        );
        // Al terminar su trayectoria libera el hueco y entra el siguiente.
        for _ in 0..8 {
            p.update(0.05);
        }
        assert_eq!(p.active_count(), 0, "el vuelo completa su trayectoria");
        p.spawn(&params, &imps, 2, false);
        assert_eq!(p.active_count(), 1, "el flujo continúa tras el impacto");
    }
}
