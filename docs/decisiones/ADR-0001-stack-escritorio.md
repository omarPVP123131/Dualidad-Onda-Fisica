# ADR-0001 · Mantener Rust + macroquad, solo escritorio

- **Estado:** Aceptada (formalización, 2026-10-16) — refleja el prototipo existente
- **Deciden:** Omar Palomares Velasco / Leonardo Miguel Vega Carbajal (confirmar en reunión)

## Contexto
El prototipo funciona en Rust con macroquad y se decidió entregar solo
escritorio (sin versión web).

## Decisión
Se mantiene Rust + macroquad 0.4 y el objetivo de escritorio
(Windows/Linux/macOS). No se migra de lenguaje ni de crate gráfico en las Fases 1–2.

## Consecuencias
- El trabajo se concentra en física, claridad y demo, no en re-aprender herramientas.
- Una futura evaluación de 3D (ADR-0004) puede revisar esta decisión con evidencia.
