# 01 · Visión y alcance

## El proyecto en una frase

Un simulador de escritorio, en Rust, que hace visible la dualidad onda-partícula:
partículas disparadas una por una que, acumuladas, dibujan interferencia.

## El problema que resuelve

La dualidad onda-partícula es contraintuitiva y suele quedarse en una fórmula.
El simulador la convierte en una experiencia: el usuario ve impactos
individuales «aleatorios» y ve emerger el patrón. Que emerja solo, sin que nadie
lo dibuje, es el corazón del proyecto.

## Para quién es

| Público | Qué le aporta |
|---|---|
| Cualquier curioso | Clonar, `cargo run` y ver el fenómeno sin pelearse con el teclado |
| Estudiantes (usuarios) | Manipular λ, `d`, `a` y el detector, y predecir antes de observar |
| Los autores | Base para aprender Rust, física computacional y luego 3D |

## Alcance actual (v0.7.0) ✅

- Simulador de escritorio (Windows/Linux/macOS vía Rust + macroquad).
- Modelo analítico de Fraunhofer, muestreo por rechazo, modo detector.
- Escena 3D orbitable con panel de control, presets, 1–8 rendijas y HUD.
- Controles de teclado y ratón documentados en el README.

## Fuera de alcance (por ahora) 🚫

- Versión web/móvil (decisión tomada: **solo escritorio**).
- Schrödinger 2D en tiempo real (posible fase futura, no prometida).
- Multiusuario, cuentas, red, base de datos: no aplican a este proyecto.

## Alcance futuro: lo definen los autores

La v1 del 3D se definió en ADR-0004 y ya está implementada. Cualquier
ampliación (nuevos modos, propagación temporal, exportación) se define antes
de codearse. Cualquier fecha que se dé antes de esa definición es un invento.

## Criterios de éxito (v1.0)

1. Compila y corre en Windows y Linux (probado, no supuesto).
2. El patrón aparece sin detector y desaparece con detector — explicable en vivo.
3. Documentación que conecta fenómeno → ecuación → código → resultado observado.
4. Repositorio ordenado: README, licencia decidida, instrucciones que sí funcionan.
