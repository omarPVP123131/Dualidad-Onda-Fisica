# Licencia del proyecto

## Decisión cerrada ✅

Este proyecto se distribuye bajo la **GNU General Public License v3.0 o
posterior** — identificador SPDX: **`GPL-3.0-or-later`**.

- **Archivo oficial:** [`LICENSE`](LICENSE) (texto íntegro de GPL-3.0, FSF).
- **Titulares:** Copyright © 2026 Omar Palomares Velasco y Leonardo Miguel Vega Carbajal.
- **Decisión registrada en:** `docs/decisiones/ADR-0003-licencias.md`
  (2026-10-16, criterio: copyleft).
- **Sin datos de escuela:** materia, institución y docente quedan como
  plantilla en `docs/07-entrega-escolar.md` hasta que los autores los completen.

## Qué significa, en español claro

| Quien lo recibe SÍ puede | A cambio, DEBE |
|---|---|
| Usarlo, estudiarlo y ejecutarlo para cualquier fin | Mantener el aviso de copyright y de licencia |
| Copiarlo y compartirlo | Si distribuye una versión modificada, distribuirla también bajo GPL-3.0-or-later |
| Modificarlo y publicar sus mejoras | Publicar el código fuente de sus modificaciones y marcar los cambios |

Y **no** puede: cerrar una versión derivada y distribuirla como propietaria,
ni retirar las libertades que recibió. Eso es exactamente el copyleft que se
buscó: el simulador y sus derivados siguen siendo abiertos y educativos.

## Por qué GPL y no MIT (resumen)

MIT habría permitido que cualquier derivado se cerrara. Como la prioridad
de los autores es que el proyecto **siga siendo abierto**, se eligió GPL-3.0-or-later.
El texto alternativo de MIT que existía en la formalización inicial se retiró
para que no haya ninguna ambigüedad: **una sola licencia manda, `LICENSE`.**

## Única salvedad

Al momento de decidir se desconocía si la escuela/docente exige alguna
condición propia de licenciamiento. Si la exige, esa indicación prevalece:
consultarlo con el docente antes de la entrega y, si aplica, actualizar
ADR-0003 y este archivo en el mismo cambio.

## Nota sobre el código fuente

No se añadió la cabecera GPL dentro de `src/main.rs` porque el código quedó
congelado por diseño (solo se autorizó renombrar `Cargo.toml`). La licencia
aplica por el archivo `LICENSE` raíz. Si en el futuro retocan el código,
pueden añadir la cabecera estándar de la FSF en cada archivo, opcional.
