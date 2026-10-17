# ADR-0003 · Licencia del proyecto: GPL-3.0-or-later

- **Estado:** ✅ Aceptada
- **Fecha:** 2026-10-16
- **Deciden:** Omar Palomares Velasco / Leonardo Miguel Vega Carbajal (criterio aportado: copyleft)

## Contexto
El README original declaraba «MIT y GPL» sin archivos de licencia ni modalidad
(¿una u otra?, ¿ambas?), lo que legalmente no definía nada. Había que cerrar
el tema con una sola licencia comprobable en el repositorio.

## Decisión
El proyecto se licencia **solo** como **`GPL-3.0-or-later`**
(GNU General Public License v3.0 o posterior). El texto íntegro oficial
vive en `LICENSE`. El archivo alternativo de MIT se retiró del workspace.

Copyright © 2026 Omar Palomares Velasco y Leonardo Miguel Vega Carbajal.

## Alternativas consideradas
| Opción | A favor | En contra | Resultado |
|---|---|---|---|
| Solo MIT | Máxima difusión, simplicidad | Un derivado distribuido puede cerrarse | Descartada |
| Dual `MIT OR GPL-3.0` | Quien participa elige | Más compleja de explicar y de mantener clara | Descartada |
| **Solo GPL-3.0-or-later** | Los derivados distribuidos se mantienen abiertos (copyleft) | Exige publicar fuente de derivados: es justo la intención | **Elegida** |

## Consecuencias
- Quien distribuya versiones modificadas debe hacerlo bajo GPL-3.0-or-later, con fuente.
- README y `LICENCIAS.md` declaran una sola licencia: sin ambigüedad para terceros.
- Salvedad retirada (2026-10-17): el proyecto pasó a ser personal, no escolar;
  no existe requisito externo de licenciamiento.
