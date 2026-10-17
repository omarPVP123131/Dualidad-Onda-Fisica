# Dualidad Onda-Partícula

Simulador interactivo del **experimento de la doble rendija**: partículas disparadas
**una por una** que, al acumularse, forman un patrón de interferencia ondulatorio.

> **Estado del proyecto:** prototipo funcional v0.1.0 (2D) — código congelado.
> La siguiente etapa (definición y desarrollo 3D) la realizarán los autores.
> Consulta el estado real siempre en [`docs/05-roadmap.md`](docs/05-roadmap.md).

Repositorio original: [omarPVP123131/Dualidad-Onda-Fisica](https://github.com/omarPVP123131/Dualidad-Onda-Fisica)

## Autores

- Omar Palomares Velasco
- Leonardo Miguel Vega Carbajal

Proyecto escolar/universitario. Materia, institución, grupo y docente: **por completar**
en [`docs/07-entrega-escolar.md`](docs/07-entrega-escolar.md).

## Qué hace hoy (v0.1.0)

- Doble rendija con difracción de Fraunhofer: envolvente de una rendija × interferencia de dos.
- Disparo partícula por partícula; cada impacto se muestrea según |ψ|² (muestreo por rechazo).
- Pantalla de detección con cada impacto individual.
- Histograma de impactos comparado en vivo con la curva teórica.
- Detector *which-path*: al activarlo (`O`) desaparece la interferencia.
- Color de los impactos según la longitud de onda λ.

La explicación física completa está en [`docs/02-fisica.md`](docs/02-fisica.md).
**En esta formalización no se modificó la física ni el código.**

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (toolchain estable) y `Cargo`.
- En Linux pueden requerirse librerías de desarrollo X11/OpenGL y ASLA; el detalle
  por sistema operativo está en [`docs/04-instalacion-y-desarrollo.md`](docs/04-instalacion-y-desarrollo.md).

## Instalación y ejecución

```bash
git clone https://github.com/omarPVP123131/Dualidad-Onda-Fisica.git
cd Dualidad-Onda-Fisica
Cargo run --release
```

> En la formalización se renombró el manifiesto a `Cargo.toml` (con mayúscula,
> como busca Cargo en Linux/macOS/CI). El contenido del archivo no cambió ni
> una línea. Detalle en `CHANGELOG.md` y `docs/04-instalacion-y-desarrollo.md`.

## Controles actuales

| Tecla | Acción |
|---|---|
| `↑` / `↓` | Aumentar / disminuir λ |
| `←` / `→` | Disminuir / aumentar la separación `d` |
| `W` / `S` | Aumentar / disminuir el ancho `a` |
| `O` | Activar / desactivar el detector *which-path* |
| `+` / `-` | Duplicar / reducir el ritmo de disparo |
| `Espacio` | Pausar / reanudar |
| `R` | Reiniciar la pantalla |

## Documentación

Empieza por el índice: [`docs/00-indice.md`](docs/00-indice.md)

| Documento | Para qué sirve |
|---|---|
| `docs/01-vision-y-alcance.md` | Qué es, qué no es y para quién |
| `docs/02-fisica.md` | Modelo, ecuaciones, supuestos y límites |
| `docs/03-arquitectura-actual.md` | Mapa honesto del código actual, línea por línea |
| `docs/04-instalacion-y-desarrollo.md` | Instalar, correr y diagnosticar |
| `docs/05-roadmap.md` | **Roadmap por fases hasta la entrega y el 3D** |
| `docs/06-diseno-3d.md` | Plantilla para que los autores definan el 3D |
| `docs/07-entrega-escolar.md` | Checklist de entrega, informe y demo |
| `docs/08-riesgos-y-deuda-tecnica.md` | Riesgos y deuda técnica sin maquillaje |
| `docs/09-preguntas-abiertas.md` | **Preguntas que los autores deben responder** |
| `docs/10-glosario-y-referencias.md` | Glosario y bibliografía |
| `docs/decisiones/` | Registro de decisiones (ADR) |

## Estructura

```
.
├── Cargo.toml / Cargo.lock   # manifiesto (renombrado) y versiones fijadas
├── src/main.rs               # todo el simulador actual (sin cambios)
├── docs/                     # documentación, roadmap, decisiones y plantillas
├── LICENSE                   # GPL-3.0 (texto oficial íntegro)
└── LICENCIAS.md              # la licencia explicada en español claro
```

## Licencia

**GNU GPL v3.0 o posterior** (`GPL-3.0-or-later`) — ver [`LICENSE`](LICENSE).
Copyright © 2026 Omar Palomares Velasco y Leonardo Miguel Vega Carbajal. Quien distribuya
versiones modificadas debe mantenerlas bajo la misma licencia (copyleft).
Explicado en español claro en [`LICENCIAS.md`](LICENCIAS.md) y decidido en
`docs/decisiones/ADR-0003-licencias.md`.
