# ADR-0006 · Sliders reinician el experimento y la pantalla no se actualiza en tiempo real

- **Estado:** Aceptado
- **Fecha:** 2026-10-17
- **Deciden:** Omar / Leonardo

## Contexto

Al ajustar parámetros físicos con los sliders del panel de control (longitud de onda λ, separación entre rendijas d, ancho de cada rendija a, distancia a la pantalla L), el experimento se reiniciaba por completo y volvía a cero colisiones. Además, al ajustar el ancho de la rendija a un valor bajo, los fotones seguían disparando pero la pantalla de impacto (pared final) no se actualizaba en tiempo real: los puntos aparecían de golpe tras ciertos momentos en lugar de fluir continuamente.

## Síntomas

1. **Reinicio al mover sliders:** cambiar cualquier parámetro físico llamaba `sim.reset()`, borrando histograma, impactos y partículas en vuelo. Solo el botón "↺ Reiniciar" debería hacer eso.
2. **Pantalla sin actualización en tiempo real:** la malla de impactos sobre la pantalla solo se reconstruía cada 250 impactos nuevos (`REBUILD_EVERY = 250`), provocando que los puntos aparecieran por lotes en lugar de uno a uno.

## Causa raíz

### Síntoma 1 — Sliders reinician el experimento

En `crates/dualidad-engine3d/src/controls.rs`, cada slider de parámetro físico ejecutaba:

```rust
sim.params.set_wavelength_nm(...);
sim.reset();
out.reset_visuals = true;
```

Esto forzaba un reinicio completo de la simulación (histograma e impactos a cero) cada vez que se movía un slider, incluso sin que el usuario pidiera reiniciar.

### Síntoma 2 — Pantalla sin actualizar en tiempo real

En `crates/dualidad-engine3d/src/impacts.rs`, la constante de reconstrucción de la malla estaba fijada en:

```rust
const REBUILD_EVERY: usize = 250;
```

La caché solo se reconstruía cuando se acumulaban 250 impactos nuevos, por lo que entre reconstrucciones la pantalla permanecía estática y los puntos aparecían de golpe cada cierto tiempo.

## Decisión

### Para el síntoma 1

Quitar `sim.reset()` y `out.reset_visuals = true` de los cuatro sliders de parámetros físicos (λ, d, a, L) en `controls.rs`. Ahora los cambios de parámetros se aplican al instante sin borrar el progreso. El botón "↺ Reiniciar" sigue disponible para limpiar todo cuando el usuario lo desee.

### Para el síntoma 2

Bajar `REBUILD_EVERY` de `250` a `25` en `impacts.rs`. La malla de impactos se reconstruye mucho más seguido, haciendo que los puntos aparezcan en la pantalla en tiempo real a medida que los fotones impactan.

## Alternativas consideradas

| Error | Opción | A favor | En contra |
|---|---|---|---|
| Sliders reinician | Quitar solo `sim.reset()` de los sliders | Cambio mínimo; los parámetros aplican en caliente | El usuario debe reiniciar manualmente si quiere limpiar todo |
| Sliders reinigan | Dejar como está | El histograma siempre coincide con los parámetros actuales | Rompe la experiencia de ajuste progresivo |
| Pantalla lenta | Bajar `REBUILD_EVERY` a 25 | Actualización visual fluida en tiempo real | Ligeramente más reconstrucciones de malla (impacto despreciable en rendimiento) |
| Pantalla lenta | Eliminar `REBUILD_EVERY` (reconstruir cada frame) | Máxima fluidez visual | Coste de reconstrucción de malla cada frame; innecesario |

## Consecuencias

Positivas:
- Los sliders cambian parámetros físicos sin reiniciar el experimento.
- La pantalla de impactos se actualiza en tiempo real a medida que llegan los fotones.
- La experiencia de ajuste progresivo es más fluida y predecible.

Negativas / compromisos:
- Ninguna significativa. El costo de reconstruir la malla cada 25 impactos es despreciable frente al beneficio visual.

## Pruebas sugeridas

- Mover cada slider (λ, d, a, L) durante una ejecución y verificar que el histograma y el conteo de impactos no se reinician.
- Ajustar el ancho de la rendija al mínimo y disparar a rate alto; verificar que los puntos aparecen en la pantalla de forma continua, no por lotes.
- Ejecutar `cargo test --workspace` y `cargo clippy -- -D warnings` para confirmar que no hay regressiones.
