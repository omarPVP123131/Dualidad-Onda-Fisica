# 05 · Roadmap

> Un roadmap útil distingue **hecho**, **definido por hacer** y **sin definir**.
> La fase 3D está, a propósito, en «sin definir»: la definen los autores.

## Estado por fases

| Fase | Contenido | Estado |
|---|---|---|
| 0 | Formalización del workspace (esta entrega) | ✅ Hecha (2026-10-16) |
| 1 | Base técnica sana — sin cambiar el comportamiento visible | 📋 Definida |
| 2 | Experiencia 2D lista para entregar en clase | 📋 Definida |
| 3 | **Diseño y desarrollo 3D** | 🟢 Primera versión implementada (motor y escena base) |
| 4 | Pulido y entrega escolar | 📋 Definida (fecha por confirmar) |
| 5 | Futuro opcional (post-entrega) | 💡 Ideas, sin compromiso |

---

## Fase 0 · Formalización ✅ (2026-10-16)

- [x] Clonar y leer el prototipo; código congelado y verificado por SHA-256.
- [x] Documentación completa, roadmap, ADR, glosario y referencias.
- [x] Textos de licencia en el repo + guía de decisión (`LICENCIAS.md`).
- [x] Plantillas de informe, CI, issues y PR.

## Fase 1 · Base técnica sana 📋

*Objetivo: el mismo simulador, pero construible y comprobable en cualquier
máquina. Nada de esto cambia lo que el usuario ve.*

- [x] Renombrar `cargo.toml` → `Cargo.toml` (2026-10-16; contenido intacto — doc. 04, punto 4).
- [ ] Compilar desde cero en Windows **y** en Linux; completar el doc. 04 con lo aprendido.
- [x] Separar por crates sin cambiar la física: core / engine3d / app (v0.2.0, doc. 03).
- [x] Tests de la física: máximo, simetría, modo detector, rango [0, 1] (25 tests en total).
- [ ] Activar la CI propuesta en `docs/plantillas/ci.yml` (fmt + clippy + test + build).
- [x] Licencia decidida: `GPL-3.0-or-later` (ADR-0003); una sola licencia declarada en README y `LICENSE`.

**Criterio de salida:** clon nuevo + `cargo run --release` funciona en ambas
máquinas, con tests en verde, sin que la simulación se vea distinta.

## Fase 2 · Experiencia 2D para clase 📋

*Objetivo: que el profe vea el fenómeno sin que ustedes peleen con el teclado.*

- [ ] Probar la demo completa con el guión del doc. 02 (sin detector → con detector).
- [ ] Capturas oficiales en `docs/assets/` (patrón emergiendo, modo detector).
- [ ] Revisar textos en pantalla: ¿se leen a 3 metros en un proyector?
- [ ] Completar el informe escolar con la plantilla de `docs/plantillas/`.
- [ ] Plan B de demo: video corto grabado por si la máquina del salón falla.

**Criterio de salida:** un compañero que nunca vio el proyecto puede correrlo
y explicar el fenómeno solo con el README y el informe.

## Fase 3 · 3D 🟢 PRIMERA VERSIÓN IMPLEMENTADA — pulir con los autores

- [x] Definición v1: superficie `I(x)` en laboratorio 3D orbitable (ADR-0004).
- [x] Arquitectura por crates: `dualidad-core` / `dualidad-engine3d` / `dualidad-app`.
- [x] Escena: fuente, doble rendija, sensores, pantalla, impactos, histograma 3D y HUD.
- [x] Verificación: tests, clippy limpio, arranque headless sin errores y captura visual revisada.
- [x] v0.3: UI por ratón (sliders/vistas/capas/presentación), presets fotón/electrón, 1–8 rendijas, CI activa y GIF.
- [ ] Jugarla en las máquinas de los autores y ajustar cámara/escala/ritmo con su feedback.
- [ ] Capturas oficiales del 3D en `docs/assets/` para el informe.
- [ ] Ideas siguientes (si sobra tiempo): presets, control de `L`, exportación.

## Fase 4 · Entrega escolar 📋 (fecha: **por confirmar**)

- [ ] Completar datos y checklist de `docs/07-entrega-escolar.md`.
- [ ] Ensayo cronometrado de la exposición con preguntas difíciles
  (¿por qué desaparece el patrón?, ¿qué supuestos usa el modelo?).
- [ ] Congelar versión de entrega: tag de git `v1.0-entrega` (o el que aplique).
- [ ] Copia de respaldo del ejecutable y del informe fuera del repo.

## Fase 5 · Futuro opcional 💡

*Ideas heredadas del prototipo; ninguna está comprometida:*

- Resolver Schrödinger 2D (split-step/FFT) y ver propagar el paquete de ondas.
- Borrador cuántico (*quantum eraser*), N rendijas y redes de difracción.
- Presets de partículas reales con de Broglie (fotón, electrón a X energía).
- Exportar impactos (CSV) y capturas (PNG) para informes.

---

## Cómo mantener este roadmap

- Marcar `[x]` al terminar, con la fecha en el mismo renglón si ayuda.
- Si una tarea se abandona, no se borra: se tacha y se anota el porqué.
- Toda fase nueva nace con su criterio de salida. Sin criterio, no es fase: es deseo.
