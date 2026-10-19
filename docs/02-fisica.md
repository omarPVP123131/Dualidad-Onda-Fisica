# 02 · La física del simulador

> Este documento describe el modelo **tal como está implementado** en la v0.1.0.
> No se cambió ninguna fórmula en la formalización. Si el código cambia,
> este documento se actualiza en el mismo PR.

## El fenómeno

En el experimento de la doble rendija, una fuente emite partículas (fotones,
electrones…) hacia una barrera con dos rendijas y una pantalla trasera.
Disparadas **una por una**, cada una llega como un punto. Con el tiempo, los
puntos acumulados forman franjas de interferencia: el comportamiento de una
onda… construida por partículas.

## Modelo implementado (Fraunhofer, ángulos pequeños)

Intensidad relativa en un punto `x` de la pantalla, a distancia `L` de las rendijas:

$$ I(x) = \mathrm{sinc}^2(\beta)\;\cos^2(\delta), \qquad
\beta = \frac{\pi a x}{\lambda L}, \quad
\delta = \frac{\pi d x}{\lambda L} $$

| Símbolo | Significado | Valor inicial en el código |
|---|---|---|
| `λ` | Longitud de onda | 550 nm (visible, verde) |
| `d` | Separación entre rendijas | 60 µm |
| `a` | Ancho de cada rendija | 10 µm |
| `L` | Distancia rendijas → pantalla | 1.0 m (fija en la v0.1.0) |
| `x` | Posición en la pantalla | de −20 mm a +20 mm |

- `sinc²(β)` — envolvente de difracción de **una** rendija (con `sinc(0)=1`).
- `cos²(δ)` — interferencia entre **las dos** rendijas.
- El producto: franjas finas moduladas por una envolvente ancha.

## Detector *which-path*

Si un detector permite saber por cuál rendija pasó cada partícula, la
coherencia se pierde y el término de interferencia desaparece:

$$ I_{\text{detector}}(x) = \mathrm{sinc}^2(\beta) $$

En el simulador es la tecla `O`. Es, didácticamente, el experimento completo:
**misma fuente, mismas rendijas; solo cambia si se observa el camino.**

## Cómo nace cada impacto (muestreo por rechazo)

1. Se propone un `x` uniforme al azar dentro de la pantalla.
2. Se acepta con probabilidad `I(x)`; si no, se reintenta.
3. El impacto aceptado suma 1 en su *bin* del histograma y se dibuja como punto.

Ninguna partícula «sabe» dónde caer: el patrón es un resultado estadístico,
emergente tras cientos o miles de impactos. Así se comporta el sistema real.

## Longitud de onda y color

El color de los impactos se deriva de λ dentro del visible (380–750 nm),
mapeando el espectro de forma aproximada (violeta → azul → verde → amarillo → rojo).

## Relación de de Broglie (contexto)

Para materia, $\lambda = h/p$. El simulador actual trabaja con λ directamente;
usar partículas concretas (electrón a cierta energía, fotón de cierta frecuencia)
es una posible mejora didáctica **futura**, registrada en el roadmap — no está implementada.

## Supuestos y limitaciones (léanlos antes de la defensa)

- **Aproximación de Fraunhofer / campo lejano y de ángulos pequeños**: vale
  cuando `L` es grande frente a `d` y al tamaño de la rendija. No modela campo cercano (Fresnel).
- Fuente puntual, monocromática y coherente; las rendijas son ideales y 1D,
  no se abren o cierran físicamente en pantalla.
- `I(x)` es intensidad **relativa** (máximo 1): no hay fotones «reales» ni unidades de energía.
- El azar viene del generador pseudoaleatorio de macroquad: sirve para
  enseñar el fenómeno, no es una simulación Monte Carlo de precisión metrológica.
- No se resuelve la ecuación de Schrödinger: es un modelo analítico,
  instantáneo y estable, a cambio de no mostrar la propagación de la onda.

## Qué mostrar en la exposición (guión de 60 segundos)

1. «Cada punto es UNA partícula. Nadie decide dónde cae.»
2. Esperar a que emerja el patrón. «Esto es interferencia, sin ninguna onda dibujada.»
3. Pulsar `O`. «Ahora sabemos por cuál rendija pasó… y el patrón murió.»
4. Quitar `O` y reiniciar (`R`). «La información del camino es lo que importa.»
