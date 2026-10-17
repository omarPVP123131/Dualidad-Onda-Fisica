# ADR-0004 · Representación y tecnología 3D

- **Estado:** ✅ Aceptada e implementada (primera versión)
- **Fecha:** 2026-10-16
- **Deciden:** Omar Palomares Velasco / Leonardo Miguel Vega Carbajal (dirección) — implementación autorizada al asistente

## Contexto
Se deseaba una vista 3D del experimento manteniendo Rust, macroquad y el
objetivo de escritorio (ADR-0001), sin reescribir el proyecto en otro motor.

## Decisión
Motor 3D = **el propio macroquad** (`Camera3D`, mallas y primitivas 3D),
organizado en un crate separado (`dualidad-engine3d`) sobre el núcleo físico
puro (`dualidad-core`). Representación elegida:

- escena de laboratorio orbitable: fuente, barrera con dos rendijas
  (geometría transversal exagerada, declarada esquemática), sensores y pantalla;
- superficie 3D de la distribución `I(x)` desde las rendijas a la pantalla;
- impactos reales de la simulación como puntos en la pantalla 3D;
- histograma 3D frente a la pantalla con la curva teórica en línea;
- partículas en vuelo como recurso visual esquemático (no como trayectoria medida).

## Alternativas consideradas
| Opción | Resultado |
|---|---|
| Bevy u otro engine | Descartada: reescritura total y curva de aprendizaje |
| Mantener solo 2D | Descartada: era el objetivo de esta etapa |
| **macroquad 3D sobre crates propios** | **Elegida: cero migraciones, física intacta y testeable** |

## Consecuencias
- Límites prácticos detectados en la verificación: cada geometría de
  macroquad se recorta a 5 000 índices, por lo que la superficie y los
  impactos se emiten en mallas por lotes/tiras (probado en ejecución).
- Ajustar o sustituir la representación ya no toca la física: cambiará
  `dualidad-engine3d`, no `dualidad-core`.
