# Dualidad Onda-Partícula

Simulador interactivo del **experimento de la doble rendija**, escrito en Rust con [macroquad](https://github.com/not-fl3/macroquad). Muestra cómo las partículas, disparadas **una por una**, llegan a la pantalla en puntos aparentemente aleatorios y, aun así, terminan formando un patrón de interferencia propio de una onda.

Repositorio: [omarPVP123131/Dualidad-Onda-Fisica](https://github.com/omarPVP123131/Dualidad-Onda-Fisica)

## Características

- Doble rendija con difracción de Fraunhofer (envolvente de una rendija × interferencia de dos).
- Disparo partícula por partícula: cada impacto se muestrea con la distribución de probabilidad |ψ|².
- Pantalla de detección donde se ve cada impacto individual.
- Histograma de impactos comparado en vivo con la curva teórica.
- Detector *which-path*: al activarlo desaparece la interferencia.
- Color de los impactos según la longitud de onda.
- Compilación rápida y bajo consumo de recursos.

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (toolchain estable) y `cargo`.
- En Linux pueden hacer falta las librerías de desarrollo de X11/OpenGL y ALSA (por ejemplo `libasound2-dev`).

## Instalación y ejecución

```bash
git clone https://github.com/omarPVP123131/Dualidad-Onda-Fisica.git
cd Dualidad-Onda-Fisica
cargo run --release
```

Durante el desarrollo, `cargo run` compila rápido porque las dependencias se optimizan y solo tu código va en modo debug.

## Controles

| Tecla | Acción |
|-------|--------|
| `↑` / `↓` | Aumentar / disminuir la longitud de onda λ |
| `←` / `→` | Disminuir / aumentar la separación entre rendijas `d` |
| `W` / `S` | Aumentar / disminuir el ancho de cada rendija `a` |
| `O` | Activar / desactivar el detector *which-path* |
| `+` / `-` | Duplicar / reducir el ritmo de disparo |
| `Espacio` | Pausar / reanudar |
| `R` | Reiniciar la pantalla |

## La física

La intensidad relativa en un punto `x` de la pantalla, a distancia `L` de las rendijas, es:

```
I(x) = sinc²(β) · cos²(δ)

β = π · a · x / (λ · L)      (difracción de cada rendija)
δ = π · d · x / (λ · L)      (interferencia entre las dos rendijas)
```

donde `λ` es la longitud de onda, `d` la separación entre rendijas y `a` el ancho de cada una (aproximación de ángulos pequeños).

- **Sin detector:** se observa el patrón completo de franjas (interferencia) modulado por la envolvente de difracción.
- **Con detector (`O`):** se conoce por cuál rendija pasó cada partícula, el término `cos²(δ)` desaparece y queda solo la envolvente `sinc²(β)`.

Para materia, la longitud de onda asociada se obtiene con la relación de de Broglie, `λ = h / p`.

Cada impacto se genera por **muestreo por rechazo** sobre `I(x)`: ninguna partícula individual "sabe" dónde caerá, pero la acumulación reproduce la distribución teórica.

## Estructura del proyecto

```
.
├── Cargo.toml
└── src
    └── main.rs
```

## Ideas futuras

- Resolver la ecuación de Schrödinger dependiente del tiempo en 2D (split-step con FFT) en lugar de la fórmula analítica.
- Vista 3D con |ψ|² como malla de altura y la fase como color.
- Borrador cuántico (*quantum eraser*).
- Más rendijas, redes de difracción y obstáculos personalizados.

## Autores

- Omar Palomares Velasco
- Leonardo Miguel Vega Carbajal

## Licencia

Distribuido bajo las licencias **MIT** y **GPL**.