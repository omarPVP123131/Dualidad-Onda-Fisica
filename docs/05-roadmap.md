# 05 · Roadmap

> Un roadmap útil distingue **hecho**, **definido por hacer** y **sin definir**.
> Las fases futuras las definen los autores; nada entra al alcance por deseo.

## Estado por fases

| Fase | Contenido | Estado |
|---|---|---|
| 0 | Formalización del workspace | ✅ Hecha (2026-10-16) |
| 1 | Base técnica sana — sin cambiar el comportamiento visible | ✅ Hecha |
| 2 | Experiencia 2D pulida | ✅ Superada por el 3D |
| 3 | **Diseño y desarrollo 3D** | 🟢 Primera versión implementada — puliendo con uso real |
| 4 | Pulido y versión 1.0 | 📋 Definida |
| 5 | Futuro opcional | 💡 Ideas, sin compromiso |

---

## Fase 0 · Formalización ✅ (2026-10-16)

- [x] Clonar y leer el prototipo; código congelado y verificado por SHA-256.
- [x] Documentación completa, roadmap, ADR, glosario y referencias.
- [x] Textos de licencia en el repo + guía de decisión (`LICENCIAS.md`).
- [x] Plantilla de CI, issues y PR.

## Fase 1 · Base técnica sana ✅

*Objetivo cumplido: el mismo simulador, pero construible y comprobable en
cualquier máquina, sin cambiar lo que el usuario ve.*

- [x] Renombrar `cargo.toml` → `Cargo.toml` (2026-10-16; contenido intacto — doc. 04).
- [x] Compilar desde cero en Windows **y** en Linux (local + CI).
- [x] Separar por crates sin cambiar la física: core / engine3d / app (v0.2.0, doc. 03).
- [x] Tests de la física y la escena: 41 en verde (doc. 03).
- [x] CI activa: fmt + clippy + test + build y humo de arranque (`.github/workflows/ci.yml`).
- [x] Licencia decidida: `GPL-3.0-or-later` (ADR-0003); una sola licencia declarada en README y `LICENSE`.

**Criterio de salida:** clon nuevo + `cargo run --release` funciona en ambas
máquinas, con tests en verde, sin que la simulación se vea distinta. ✅

## Fase 2 · Experiencia 2D pulida ✅

*Superada por el 3D: la escena 3D reemplazó al panel 2D como experiencia principal.*

- [x] Demo completa con el guión del doc. 02 (sin detector → con detector).
- [x] Capturas oficiales en `docs/assets/` (patrón emergiendo, modo detector).
- [x] Textos en pantalla legibles en proyector o pantalla compartida.
- [ ] Video corto de la demo, de respaldo para compartir (opcional).
- ~~Informe escolar~~: retirado el 2026-10-17 (el proyecto es personal).

**Criterio de salida:** quien nunca vio el proyecto puede correrlo y explicar
el fenómeno solo con el README. ✅

## Fase 3 · 3D 🟢 PRIMERA VERSIÓN IMPLEMENTADA — pulir con los autores

- [x] Definición v1: superficie `I(x)` en laboratorio 3D orbitable (ADR-0004).
- [x] Arquitectura por crates: `dualidad-core` / `dualidad-engine3d` / `dualidad-app`.
- [x] Escena: fuente, doble rendija, sensores, pantalla, impactos, histograma 3D y HUD.
- [x] Verificación: tests, clippy limpio, arranque headless sin errores y captura visual revisada.
- [x] v0.3: UI por ratón (sliders/vistas/capas/presentación), presets fotón/electrón, 1–8 rendijas, CI activa y GIF.
- [x] v0.7.0: cámara WASD, barra de desplazamiento y correcciones de uso real.
- [ ] Jugarla en las máquinas de los autores y ajustar cámara/escala/ritmo con su feedback.
- [ ] Capturas oficiales del 3D en `docs/assets/` para el README.
- [ ] Ideas siguientes (si sobran ganas): control de `L`, exportación.

## Fase 4 · Pulido y versión 1.0 📋

- [ ] Cerrar el pulido de cámara, textos y rendimiento con uso real.
- [ ] Actualizar ESTADO, docs y capturas que hayan cambiado.
- [ ] Congelar versión: tag de git `v1.0`.
- [ ] Copia de respaldo del ejecutable (Windows/Linux) fuera del repo.

## Fase 5 · Futuro opcional 💡

*Ideas heredadas del prototipo; ninguna está comprometida:*

- Resolver Schrödinger 2D (split-step/FFT) y ver propagar el paquete de ondas.
- Borrador cuántico (*quantum eraser*), N rendijas y redes de difracción.
- Exportar impactos (CSV) y capturas (PNG).

---

## Cómo mantener este roadmap

- Marcar `[x]` al terminar, con la fecha en el mismo renglón si ayuda.
- Si una tarea se abandona, no se borra: se tacha y se anota el porqué.
- Toda fase nueva nace con su criterio de salida. Sin criterio, no es fase: es deseo.
