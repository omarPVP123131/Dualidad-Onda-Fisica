# ADR-0008 · Rendimiento: lotes de geometría y anillo incremental de impactos

- **Estado:** Aceptado
- **Fecha:** 2026-10-18
- **Deciden:** Omar / Leonardo

## Contexto

Petición de uso real: «¿podemos optimizar más el proyecto para más fps?».
El simulador debe ir fluido también en equipos modestos o sin GPU (render
por software), que es donde más se nota.

## Diagnóstico

En macroquad **cada** `draw_line_3d`, `draw_cube`, `draw_sphere` o
`draw_cylinder` sube su propia geometría al GPU: no existe *batching* 3D.
Con el medidor de llamadas por frame (nivel Alta, escena viva):

| Foco | Subidas de geometría por frame |
|---|---|
| Efectos de onda (contornos ~980, frentes ~300, perfil ~270) | ~1 550 |
| Escena del laboratorio (estática salvo parámetros) | ~300 |
| Partículas (bloques + rastros; esferas de ~256 triángulos) | hasta ~480 |
| Impactos | ~5–100 mallas ✅ (ya por lotes) |

Además, la caché de impactos se **reconstruía entera** cada 25 impactos
nuevos (hasta 40 000 quads ≈ 160 000 vértices más asignaciones) y volvía a
reconstruirse al deslizarse la ventana del almacén, con picos de CPU en
máquinas lentas.

## Decisión

- **`batch.rs` (nuevo)**: lotes reutilizables. `MeshBatch` acumula
  triángulos en mallas de ≤4 800 índices (el tope de macroquad es 5 000);
  `LineBatch` convierte polilíneas en cintas orientadas a la vista; los
  búferes se conservan entre frames (cero asignaciones en el bucle).
- **Efectos de onda como cintas**: contornos, frentes animados y perfil
  viajan en un solo `LineBatch` por frame (~1 550 subidas → ~3).
- **Escena estática en mallas cacheadas** (`scene::build_lab_meshes`) con
  clave `LabKey` (λ, `d`, `a`, nº de rendijas, detalle ambiental). El
  detector **no** entra en la clave: su luz de estado va en
  `draw_lab_extras`, así alternarlo no reconstruye nada. Solo quedan
  sueltas las piezas redondas (esferas y cilindros, una docena).
- **Partículas en dos lotes** (`MeshBatch` + `LineBatch`): en Alta el
  bloque es un disco orientado a la cámara — misma silueta que la esfera,
  8 triángulos en vez de ~256 —; en el resto, cubos como antes. Los
  rastros pasan de línea por trazo a cinta en el lote.
- **Anillo incremental de impactos**: la caché pasa a ser un anillo de
  ranuras fijas (`slots` = tope de dibujo del nivel). Cada impacto nuevo
  escribe sus 4 vértices (núcleo) y 4 (halo) en O(1); ya no hay
  reconstrucción completa ni muestreo intermedio de 1 MB. La ventana
  deslizante del núcleo puede recortar por delante sin tocar la caché: el
  anillo ya guarda los más recientes. Solo se repinta entero al cambiar
  de nivel, al reiniciar o al cambiar λ (en ese caso, repintado **en su
  sitio**, sin reasignar mallas).
- La física y los datos no cambian: solo cambia **cómo** se dibujan.

## Alternativas consideradas

| Opción | A favor | En contra |
|---|---|---|
| Solo bajar topes y `render_scale` | Trivial | Menos detalle sin ganar fluidez real: el coste está en las subidas de geometría |
| Reconstrucción de impactos cada N impactos (anterior) | Simple | Picos de CPU y asignaciones; la ventana deslizante forzaba reconstrucciones |
| Anillo incremental (elegida) | O(1) por impacto, sin picos ni asignaciones | En Alta saturada se dibujan hasta 40 000 puntos (antes el muestreo accidental dejaba ~20 500): es el tope que el nivel ya declara |
| Teselar esferas y cilindros de la escena a mano | Unas llamadas menos | Poco valor: ya son pocas y el código se vuelve ilegible |

## Consecuencias

Positivas:

- De ~2 500–3 000 subidas de geometría por frame a unas **decenas**; en
  render por software, donde cada subida es cara, es la mayor ganancia
  disponible sin tocar la física.
- La pantalla de impactos refleja cada impacto **al instante** (antes, como
  mucho, cada 25) y sin picos de reconstrucción.
- Menos asignaciones por frame: el recolector no trabaja en el bucle.

Negativas / compromisos:

- Contornos, frentes y rastros son ahora cintas de ancho en unidades del
  mundo (antes líneas de 1 px): a la distancia por defecto equivalen a
  ~1 px, de cerca se ven algo más gruesas; de lejos, más finas.
- Con Alta saturada el anillo mantiene el tope completo de 40 000 puntos
  dibujados (el nivel ya documentaba ese tope).

## Pruebas sugeridas

- `cargo test --workspace`: `los_lotes_respetan_el_tope_de_indices`,
  `las_lineas_se_convierten_en_cintas`, `la_escena_por_lotes_respeta_el_tope_de_indices`,
  `el_anillo_escribe_solo_los_impactos_nuevos`, `el_reinicio_limpia_la_pantalla`,
  `el_cambio_de_lambda_repinta_sin_reasignar`,
  `el_cambio_de_calidad_reajusta_el_anillo`.
- Reproducir a mano: arrastrar λ/`d`/`a` con ritmo alto y comprobar que la
  escena, la superficie y la pantalla se repintan sin tirones; alternar el
  detector no debe reconstruir el laboratorio.
- Humo: `cargo run --release` durante unos segundos sin errores en consola.
