# Registro de cambios (Changelog)

Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/).
Las versiones siguen [Versionado Semántico](https://semver.org/lang/es/) cuando
el proyecto empiece a publicarlas.

## [Sin publicar] — Formalización del workspace

### Agregado
- Documentación completa en `docs/`: visión, física, arquitectura actual,
  instalación, roadmap, diseño 3D (plantilla), entrega escolar, riesgos,
  preguntas abiertas, glosario y referencias.
- Registro de decisiones en `docs/decisiones/` (ADR-0001 a ADR-0004).
- Texto oficial de licencia en `LICENSE` y guía `LICENCIAS.md`.
- Plantillas de informe escolar y de CI en `docs/plantillas/`.
- Plantillas de issues y pull request en `.github/`.
- Copia histórica del README original en `docs/historico/`.

### Sin cambios (a propósito)
- `src/main.rs` y `Cargo.lock`: **idénticos byte por byte** al prototipo clonado.
  Verificación con SHA-256 en `ESTADO-DEL-PROYECTO.md`.
- Del manifiesto solo cambió el nombre de `cargo.toml` a `Cargo.toml`
  (contenido intacto), con autorización expresa de los autores.
- La física, los controles y el comportamiento visual no se tocaron.

### Decidido
- **Licencia: `GPL-3.0-or-later`** (copyleft). Se eligió GPL sobre MIT/dual,
  se retiró el texto MIT y la licencia única vive en `LICENSE`.
  Ver ADR-0003 y `LICENCIAS.md` (2026-10-16).
- `cargo.toml` → `Cargo.toml`: desbloquea Linux y CI sin tocar código.

### Pendiente (decisión de los autores)
- Definición del alcance 3D. Ver `docs/06-diseno-3d.md`.
- Datos de la entrega escolar y si el docente exige alguna licencia propia.

## [0.1.0] — Prototipo inicial

### Agregado
- Simulador 2D de la doble rendija en Rust con macroquad.
- Intensidad de Fraunhofer `I(x) = sinc²(β)·cos²(δ)`, muestreo por rechazo.
- Pantalla de impactos, histograma vs. teoría, detector *which-path* (`O`).
- Controles de teclado para λ, `d`, `a`, ritmo, pausa y reinicio.
