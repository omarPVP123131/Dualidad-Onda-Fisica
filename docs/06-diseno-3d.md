# 06 · Diseño 3D — plantilla para que la definan los autores

> **Estado: 🔴 SIN DEFINIR.** Este documento no diseña el 3D: es el formato
> para que Omar y Leonardo Miguel Vega Carbajal lo definan por escrito **antes** de programar
> una sola línea. Nadie debe implementar nada de aquí hasta aprobarlo (ADR-0004).

## 1. La pregunta de oro (responder en una frase)

> **En 3D, ¿qué va a VER el usuario que hoy no puede ver?**

Si la respuesta es «lo mismo, pero bonito», el 3D no aporta didáctica y hay
que repensarlo. Respuestas posibles (anoten la suya, no copien):

- [ ] La onda |ψ|² como superficie de altura entre rendijas y pantalla.
- [ ] El recorrido de cada partícula desde la fuente hasta el impacto.
- [ ] Las dos rendijas y la pantalla como escena espacial navegable.
- [ ] Otro: _______________________________________________

## 2. Qué representa cada eje (rellenar)

| Elemento | En la escena 3D será… | Escala/unidades |
|---|---|---|
| Eje de propagación (fuente → pantalla) | | |
| `|ψ(x)|` o amplitud | (altura / color / ¿?) | |
| La fase de la onda | (color / nada / ¿?) | |
| Los impactos | (puntos en la pantalla 3D) | |
| El detector | (objeto visible / solo texto / ¿?) | |

## 3. Interacción (rellenar)

- Cámara: ¿órbita con mouse? ¿vista fija? ¿primera persona? _______________
- Los parámetros λ, `d`, `a`: ¿panel, sliders, teclado, todo? _______________
- ¿El 3D reemplaza la vista 2D, conviven en pestañas o es otra pantalla? _______________
- ¿Qué pasa con el histograma y la curva teórica en modo 3D? _______________

## 4. Tecnología (elegir en ADR-0004)

| Opción | A favor | En contra |
|---|---|---|
| 3D nativo de macroquad (el mismo crate) | Cero dependencias nuevas; ya conocen la API 2D | Capacidad 3D básica; hay que validar qué da de sí |
| Otro crate/engine Rust (p. ej. Bevy) | 3D serio, cámara/ECS listos | Curva de aprendizaje y reescritura grande |
| Otra: ________ | | |

**Experimento obligatorio antes de decidir:** un prototipo desechable de
«una malla que se mueve» con la opción favorita, en una rama aparte. Si no
corre fluido en sus máquinas, no es opción — la escuela no espera a renders lentos.

## 5. Física en 3D: ¿la misma o cambia?

- [ ] Misma I(x) analítica, solo cambia la representación (recomendado para empezar).
- [ ] Se añade propagación temporal de la onda (ondas saliendo de las rendijas).
- [ ] Se cambia el modelo (entonces hay que actualizar `docs/02-fisica.md` **antes**).

## 6. Riesgos que ya se ven

| Riesgo | Mitigación acordada |
|---|---|
| El 3D se come el tiempo del informe/entrega | Primero Fase 1–2 del roadmap; el 3D no bloquea la entrega |
| Rendimiento (mallas que se recalculan por frame) | Prototipo de rendimiento antes de comprometer alcance |
| Los dos editando la misma escena | Un dueño por subsistema (ver `CONTRIBUTING.md`) |
| «Se ve increíble pero no se entiende» | Probar con un compañero: debe explicar el fenómeno tras 60 s |

## 7. Criterio para dar el diseño por DEFINIDO

- [ ] Pregunta de oro respondida en una frase.
- [ ] Tabla de ejes completa y tecnología elegida en ADR-0004.
- [ ] Prototipo desechable funcionando (o descartado con motivo escrito).
- [ ] Tareas de la Fase 3 del roadmap escritas con responsable y orden.

**Diseño definido por:** _____________ **Fecha:** _____________
