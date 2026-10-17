# ADR-0005 · Partículas en vuelo desaparecen tras saturar el almacén de impactos

- **Estado:** Abierto
- **Fecha:** 2026-10-17
- **Deciden:** Omar / Leonardo

## Contexto

Durante las animaciones de disparo continuo, las partículas en vuelo (`FlightParticles`) dejan de verse pasado cierto tiempo. La cámara, la superficie de onda y los impactos sobre la pantalla siguen funcionando; solo las partículas en vuelo desaparecen.

Este proyecto simula la dualidad onda-partícula de la doble rendija. Las partículas son un recurso visual que dibuja la trayectoria esquema de cada disparo reciente (fuente → una rendija → pantalla); no son los datos de medición, que vive en `dualidad-core`.

## Síntoma

- Al iniciar el disparo continuo se ven partículas volando de la fuente a la pantalla.
- Tras un tiempo proporcional al ritmo de disparo, no queda ninguna partícula activa.
- El problema persiste hasta que se reinician los visuales (`reset_visuals`) o se reinicia la simulación.

## Causa raíz

1. `Simulation` almacena como máximo `DEFAULT_MAX_STORED = 40 000` impactos (`crates/dualidad-core/src/simulation.rs:91`). Al alcanzar ese tope, `sim.impacts().len()` deja de crecer.
2. En `crates/dualidad-app/src/main.rs`, `renderer.notify_impacts` solo se invoca cuando `after > before`, es decir, solo si se agregaron impactos nuevos al vector (líneas 45-47 y 68-70).
3. Una vez saturado el almacén, `after == before` permanentemente, por tanto `notify_impacts` nunca se llama y `FlightParticles::spawn` deja de crear partículas (`crates/dualidad-engine3d/src/particles.rs`).
4. Las partículas existentes tienen `DURATION = 0.55 s` (`particles.rs:16`) y mueren sin reemplazarse.

Cadena de fallo: disparo continuo → almacén de impactos se llena a 40 000 → `notify_impacts` deja de dispararse → no se crean partículas nuevas → las existentes expiran a los 0,55 s → desaparecen todas.

## Decisión

Corregir el desacople entre "impactos generados por la física" e "impactos notificados al renderer". Las partículas son un recurso visual, no datos de medición, por lo que no necesitan mapear 1:1 a impactos almacenados tras el tope.

Opciones consideradas:

| Opción | A favor | En contra |
|---|---|---|
| **A) Notificar al renderer los impactos generados aunque no se almacenen** (usar el contador `hechas` de `shoot_many`) | Las partículas nunca dejan de verse; cambio localizado en `main.rs` | Tras el tope, las partículas ya no representan impactos físicos individuales (símbolo visual) |
| B) Eliminar el tope de `impacts` | Coherencia total entre datos y visuales | Riesgo de crecimiento de memoria sin cota |
| C) Mantener tope, pero reciclar/reusar el vector de impactos | Sin crecimiento de memoria | Cambio más invasivo en el núcleo (`simulation.rs`) |

**Opción elegida:** A (por ahora). Es la corrección más pequeña y de menor riesgo. El histograma y la cuenta total siguen usando todos los datos reales (no se ven afectados).

## Alternativas consideradas

- Dejarlo como deuda técnica documentada en `docs/08-riesgos-y-deuda-tecnica.md`. Descartado: rompe la experiencia visual del experimento de doble rendija.

## Consecuencias

Positivas:
- Las partículas vuelven a verse de forma continua durante el disparo, sin importar cuánto tiempo pase.
- Corrige el síntoma reportado sin tocar la física ni el histograma.

Negativas / compromisos:
- Tras el tope de 40 000 impactos almacenados, las partículas en vuelo dejan de mapear 1:1 a impactos físicos reales almacenados; pasan a ser representativas del flujo de disparo.

## Pruebas sugeridas

- Test de integración: disparar más de 40 000 impactos y verificar que `renderer.particles.active_count()` no cae a cero.
- Reproducir a mano con rate alto (2000/frame) y confirmar que las partículas persisten indefinidamente.
- Verificar que el histograma y `sim.total()` siguen contando correctamente tras el tope.
