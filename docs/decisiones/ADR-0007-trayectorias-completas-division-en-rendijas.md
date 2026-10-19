# ADR-0007 · Trayectorias completas y división de la partícula en las rendijas

- **Estado:** Aceptado
- **Fecha:** 2026-10-18
- **Deciden:** Omar / Leonardo

## Contexto

Reporte de uso real sobre las partículas en vuelo (`FlightParticles`):

1. «No terminan la trayectoria: se quedan a medias y nunca llegan a la pantalla.»
2. «Si se mueven parámetros al azar, a veces ni siquiera llegan a las rendijas.»
3. Se pide, además, que en las rendijas la partícula **se divida** y tenga
   trayectoria hasta el impacto (hoy se dibuja un solo camino).

## Síntomas

- Las partículas desaparecen a mitad de vuelo (cerca de las rendijas o poco
  antes de la pantalla) en vez de impactar.
- El punto donde desaparecen cambia con el ritmo de disparo, el nivel de
  calidad automático y el equipo: «a veces ni llegan a las rendijas».
- Mover los sliders físicos, además, reiniciaba el experimento mientras se
  arrastraba, borrando los vuelos en curso frame a frame.
- Tras unos minutos (o segundos con ritmo alto), las partículas dejaban de
  aparecer por completo y la pantalla de impactos se congelaba.

## Causa raíz

1. **Reciclaje del vuelo más antiguo** (`particles.rs`): con el búfer lleno,
   cada impacto nuevo eliminaba el vuelo más antiguo *en pleno vuelo*. A
   ritmo constante, la vida útil de un vuelo es `cap / (spawns·fps)`; con el
   nivel Ligera (tope 55) moría al ~20 % del recorrido (antes de la barrera)
   y con Equilibrada (120) justo en las rendijas. El síntoma dependía de
   fps/ritmo/calidad: «al azar».
2. **Almacén de impactos saturado** (ADR-0005): al llegar a 40 000,
   `impacts()` dejaba de crecer, `main.rs` no notificaba impactos nuevos y
   aparecían cero partículas; la malla de la pantalla tampoco se
   reconstruía.
3. **Sliders que reinician** (ADR-0006): la corrección se había perdido en la
   copia de trabajo — arrastrar un slider borraba los vuelos en curso en cada
   frame. Restituida aquí.
4. El vuelo dibujaba **una sola** rendija (la más cercana al destino), sin
   división.

## Decisión

- **Ningún vuelo se interrumpe**: con el búfer lleno se omiten los impactos
  sobrantes del frame, pero los vuelos en curso siempre completan su
  trayectoria (decisión central de este ADR).
- **División en las rendijas** con tres tramos paramétricos: fuente → centro
  de la barrera (cabeza), barrera → cada rendija (ramas), rendija → impacto
  (todas las ramas convergen en el mismo punto). Sin detector se dibujan
  todas las rendijas (superposición ilustrada); con detector *which-path* un
  único camino definido. Sigue siendo un recurso esquemático (ADR-0004).
- **Ventana deslizante del almacén** (opción C de ADR-0005, que sustituye a
  su opción A): al llenarse se descartan los impactos más antiguos por
  bloques (amortizado); los recientes siempre entran y las partículas
  vuelven a mapear 1:1 al flujo físico.
- `main.rs` notifica con el conteo devuelto por `shoot_many` tomando los
  últimos impactos (los índices `before..after` se rompen con la ventana).
- `impacts.rs` decide la reconstrucción de la malla con `sim.total()`
  (contador monótono), no con el tamaño del almacén, y restituye
  `REBUILD_EVERY = 25` de ADR-0006.
- `controls.rs` deja de reiniciar la simulación en los sliders físicos
  (restituido ADR-0006 en este árbol).

## Alternativas consideradas

| Opción | A favor | En contra |
|---|---|---|
| Mantener el reciclaje subiendo el tope | Cambio trivial | El corte a media trayectoria persiste (depende de fps/ritmo) |
| Congelar los spawns al llenar el búfer (elegida) | Todos los vuelos completan | Menos partículas simultáneas a ritmos altos |
| No dividir y solo elegir una rendija | Menos dibujo | No comunica la superposición ni lo pedido |

## Consecuencias

Positivas:
- Toda partícula que aparece llega a la pantalla; el vuelo es continuo y
  predecible con cualquier calidad/ritmo/equipo.
- El flujo nunca se detiene: ventana deslizante en el núcleo + malla de
  impactos que sigue a `total()`.
- Arrastrar sliders ya no borra el progreso ni los vuelos en curso.

Negativas / compromisos:
- A ritmos muy altos se dibujan menos partículas simultáneas que impactos
  (el flujo visible es una muestra; el conteo real no se toca).
- Con muchas rendijas (8) cada vuelo consume hasta 8 bloques del tope: menos
  vuelos concurrentes, mismo presupuesto de dibujo.

## Pruebas sugeridas

- `cargo test --workspace`: `el_tope_no_interrumpe_los_vuelos_en_curso`,
  `sin_detector_el_vuelo_se_divide_en_todas_las_rendijas`,
  `la_rama_termina_en_el_impacto`, `la_ventana_sigue_guardando_los_impactos_recientes`.
- Reproducir a mano: ritmo 2000/frame durante varios minutos; las partículas
  deben seguir apareciendo y llegar siempre a la pantalla.
- Arrastrar cada slider físico: los vuelos en curso no se borran y el
  histograma no se reinicia.
