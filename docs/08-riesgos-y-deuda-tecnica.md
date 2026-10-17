# 08 · Riesgos y deuda técnica (sin maquillaje)

> Deuda documentada = decisión informada. Deuda escondida = sorpresa el día
> de la entrega. Este archivo es la lista honesta del prototipo; arreglarla
> es trabajo de los autores, por fases (doc. 05).

## Deuda técnica actual

| # | Deuda | Impacto | Costo de arreglarla | Fase |
|---|---|---|---|---|
| D1 | ~~`cargo.toml` en minúsculas~~ ✅ | Resuelto el 2026-10-16: renombrado a `Cargo.toml` sin tocar su contenido | — | 1 |
| D2 | Todo en un solo `main.rs` | Física, entrada y dibujo acoplados; el 3D obligaría a reescribir sin red | Medio: separar módulos sin cambiar la física | 1 |
| D3 | **Sin tests** | Nadie garantiza que un cambio conserve la física correcta | Bajo: tests de `intensity` (simetría, máximo, modo detector) | 1 |
| D4 | La teoría se recalcula cada frame (500 bins + normalización) | Trabajo repetido por siempre; irrelevante hoy, molesto en 3D/mallas | Bajo: recalcular solo al cambiar parámetros | 1 |
| D5 | `L` fija y sin control | Un parámetro físico clave no se puede explorar en clase | Bajo (cuando toquen código) | 2 |
| D6 | ~~Licencia ambigua («MIT y GPL»)~~ ✅ | Resuelto el 2026-10-16: una sola licencia, `GPL-3.0-or-later` en `LICENSE` (ADR-0003) | — | 1 |
| D7 | Sin CI | «En mi máquina sí compila» el día de la entrega | Bajo: plantilla en `docs/plantillas/ci.yml` | 1 |
| D8 | Textos y ayudas solo en pantalla | El docente no tiene guía si los autores no están | Bajo: capturas + informe (Fase 2) | 2 |

## Riesgos del proyecto (no solo del código)

| Riesgo | Probabilidad | Si pasa… | Mitigación |
|---|---|---|---|
| El 3D consume el tiempo del informe | Alta si no se acota | Se entrega bonito e inexplicable | Fase 3 solo tras definirse (doc. 06); la entrega no depende del 3D |
| La máquina del salón no compila/no corre la demo | Media | Exposición sin demo en vivo | Ensayo en esa máquina + video de respaldo (doc. 07) |
| Alcance crece («y además Schrödinger…») | Alta | Nada termina | Fase 5 es *opcional y posterior* a la entrega |
| Dos autores, un solo archivo | Media | Conflictos de git constantes | Fase 1 (módulos) + flujo de `CONTRIBUTING.md` |
| Se pierde el hilo de qué decidió quién | Media | Se re-discute lo mismo cada semana | ADR en `docs/decisiones/` para cada decisión |

## Lo que NO es deuda (es diseño consciente del prototipo)

- Usar la fórmula analítica en vez de resolver Schrödinger: es la decisión
  correcta para un prototipo instantáneo y estable. Documentado en el doc. 02.
- Muestreo por rechazo: simple y correcto para I(x) con máximo 1.
- Tope de 40 000 puntos dibujados: protege el rendimiento; el histograma no se detiene.
