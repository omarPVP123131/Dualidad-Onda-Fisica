//! Núcleo del simulador de la doble rendija.
//!
//! Este crate es **puro**: no conoce macroquad, ni la ventana, ni el 3D.
//! Contiene únicamente el modelo físico, el muestreo estadístico y el
//! estado de la simulación. Eso lo hace testeable sin abrir una ventana
//! y reutilizable desde cualquier frontal (2D, 3D, CLI, tests).

pub mod color;
pub mod histogram;
pub mod params;
pub mod physics;
pub mod presets;
pub mod rng;
pub mod simulation;

pub use color::{wavelength_to_rgb, Rgb};
pub use histogram::Histogram;
pub use params::SimulationParams;
pub use presets::Preset;
pub use rng::SplitMix64;
pub use simulation::{Impact, Simulation};
