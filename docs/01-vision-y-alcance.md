# 01 · Visión y alcance

## El proyecto en una frase

Un simulador de escritorio, en Rust, que hace visible la dualidad onda-partícula:
partículas disparadas una por una que, acumuladas, dibujan interferencia.

## El problema que resuelve

La dualidad onda-partícula es contraintuitiva y en clase suele quedarse en una
fórmula. El simulador la convierte en una experiencia: el usuario ve impactos
individuales «aleatorios» y ve emerger el patrón. Que emerja solo, sin que nadie
lo dibuje, es el corazón didáctico del proyecto.

## Para quién es

| Público | Qué le aporta |
|---|---|
| Docente evaluador | Evidencia de modelo físico correcto, explicación y software funcionando |
| Estudiantes (usuarios) | Manipular λ, `d`, `a` y el detector, y predecir antes de observar |
| Los autores | Base para aprender Rust, física computacional y luego 3D |

## Alcance actual (v0.1.0) ✅

- Simulación 2D de escritorio (Windows/Linux/macOS vía Rust + macroquad).
- Modelo analítico de Fraunhofer, muestreo por rechazo, modo detector.
- Controles de teclado documentados en el README.

## Fuera de alcance (por ahora) 🚫

- Versión web/móvil (decisión tomada: **solo escritorio**).
- Schrödinger 2D en tiempo real (posible fase futura, no prometida).
- Multiusuario, cuentas, red, base de datos: no aplican a este proyecto.

## Alcance futuro: lo define los autores

La vista 3D está **deseada pero no definida**. No entra en el alcance hasta
completar `docs/06-diseno-3d.md` y aprobarlo en un ADR. Cualquier fecha que se
dé antes de esa definición es un invento.

## Criterios de éxito (para la entrega escolar)

1. Compila y corre en la computadora donde se presentará (probado, no supuesto).
2. El patrón aparece sin detector y desaparece con detector — explicable en vivo.
3. Informe que conecta fenómeno → ecuación → código → resultado observado.
4. Repositorio ordenado: README, licencia decidida, instrucciones que sí funcionan.
