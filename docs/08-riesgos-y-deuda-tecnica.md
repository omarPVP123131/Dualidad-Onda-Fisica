# 08 · Riesgos y deuda técnica (sin maquillaje)

> Deuda documentada = decisión informada. Deuda escondida = sorpresa más
> adelante. Este archivo es la lista honesta del proyecto; arreglarla
> es trabajo de los autores, por fases (doc. 05).

## Deuda técnica actual

| # | Deuda | Impacto | Costo de arreglarla | Fase |
|---|---|---|---|---|
| D1 | ~~`cargo.toml` en minúsculas~~ ✅ | Resuelto el 2026-10-16: renombrado a `Cargo.toml` sin tocar su contenido | — | 1 |
| D2 | ~~Todo en un solo `main.rs`~~ ✅ | Resuelto en v0.2.0: workspace por crates core/engine3d/app (doc. 03) | — | 1 |
| D3 | ~~**Sin tests**~~ ✅ | Resuelto: 41 tests de física, muestreo, mallas y cámara en verde | — | 1 |
| D4 | La teoría se recalcula cada frame (500 bins + normalización) | Trabajo repetido por siempre; irrelevante hoy, molesto en 3D/mallas | Bajo: recalcular solo al cambiar parámetros | 1 |
| D5 | `L` fija y sin control | Un parámetro físico clave no se puede explorar | Bajo (cuando toquen código) | 2 |
| D6 | ~~Licencia ambigua («MIT y GPL»)~~ ✅ | Resuelto el 2026-10-16: una sola licencia, `GPL-3.0-or-later` en `LICENSE` (ADR-0003) | — | 1 |
| D7 | ~~Sin CI~~ ✅ | Resuelto en v0.3.0: CI activa en `.github/workflows/ci.yml` (fmt, clippy, tests y humo) | — | 1 |
| D8 | Textos y ayudas solo en pantalla | Quien lo reciba no tiene guía si los autores no están | Bajo: capturas + README (Fase 2) | 2 |

## Riesgos del proyecto (no solo del código)

| Riesgo | Probabilidad | Si pasa… | Mitigación |
|---|---|---|---|
| El 3D consume el tiempo de todo lo demás | Alta si no se acota | Todo a medias por pulir el 3D | El pulido del 3D no bloquea la v1.0 (doc. 05) |
| La máquina de la demo no compila/no corre | Media | Demo en vivo sin funcionar | Probar en esa máquina + video de respaldo |
| Alcance crece («y además Schrödinger…») | Alta | Nada termina | Fase 5 es *opcional y posterior* a la v1.0 |
| Dos autores, un solo archivo | Media | Conflictos de git constantes | Fase 1 (módulos) + flujo de `CONTRIBUTING.md` |
| Se pierde el hilo de qué decidió quién | Media | Se re-discute lo mismo cada semana | ADR en `docs/decisiones/` para cada decisión |

## Lo que NO es deuda (es diseño consciente del prototipo)

- Usar la fórmula analítica en vez de resolver Schrödinger: es la decisión
  correcta para un prototipo instantáneo y estable. Documentado en el doc. 02.
- Muestreo por rechazo: simple y correcto para I(x) con máximo 1.
- Tope de 40 000 puntos dibujados: protege el rendimiento; el histograma no se detiene.
