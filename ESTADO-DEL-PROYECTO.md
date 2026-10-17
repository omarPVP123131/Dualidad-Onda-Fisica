# Estado del proyecto — tablero honesto

Última actualización: 2026-10-17 (crates movidos a `crates/`; prototipo
archivado en `docs/historico/main-original-v0.1.0.rs`; 41 tests en verde).
**Responsables de mantenerlo al día: los autores.** Actualícenlo en cada avance;
este archivo vale si refleja la realidad, no el deseo.

## Semáforo

| Área | Estado | Nota |
|---|---|---|
| Prototipo 2D (física + visual) | ✅ Funcional | v0.1.0, código congelado en esta entrega |
| Código (estructura) | ✅ Por crates | Workspace en `crates/`: `dualidad-core` + `dualidad-engine3d` + `dualidad-app`; 41 tests en verde |
| Documentación | ✅ Formalizada | Visión, física, arquitectura, roadmap, entrega, riesgos |
| Roadmap | ✅ Definido | `docs/05-roadmap.md` — la fase 3D falta **definirla**, no ejecutarla |
| Motor 3D + UI | 🟢 v0.3 | Escena rica, panel CONTROL por ratón, presets fotón/electrón, 1–8 rendijas, capas, vistas y presentación; CI activa y GIF en el README |
| Licencia | ✅ Decidida | **GPL-3.0-or-later** — una sola licencia en `LICENSE` (ADR-0003) |
| Entrega escolar | 🔴 Sin fecha confirmada | Completar `docs/07-entrega-escolar.md` cuanto antes |
| CI / tests automáticos | 🔴 Sin activar | Plantilla propuesta en `docs/plantillas/ci.yml` |

## Qué se hizo en la formalización (y qué no)

✅ Se clonó el repositorio y se creó este workspace formal a su lado.
✅ Se documentó todo: física, arquitectura, instalación, riesgos, preguntas.
✅ Se agregó roadmap por fases, ADR, licencias, changelog y plantillas.
🚫 En la formalización inicial no se modificó código. Después, con autorización expresa, se reconstruyó la arquitectura por crates y el motor 3D (ver CHANGELOG).
🚫 No se implementó 3D, ni UI nueva, ni cambios de física. Eso lo hacen los autores.

## Verificación de “código intacto” (SHA-256)

Comparando `proyecto-original/` (clon) contra esta carpeta, el 2026-10-16
(el prototipo se archivó después en `docs/historico/` sin tocar su contenido):

```
docs/historico/main-original-v0.1.0.rs   6a2c9267bc11f76bac6a0d70e02c339c3556c8978ff2b56afc18827e70f32a50  (idéntico)
Cargo.toml                               f3409c555090755766bb532a2e03cdeac17d4081798e29e8fcaac88407f95fde  (contenido idéntico; solo cambió el nombre del archivo)
```

Para re-verificar en cualquier momento:

```bash
sha256sum docs/historico/main-original-v0.1.0.rs
# En PowerShell (Windows): Get-FileHash docs\historico\main-original-v0.1.0.rs -Algorithm SHA256
```

## Los 5 siguientes pasos (en orden)

1. [ ] Completar los datos de entrega (materia, docente, fecha) en `docs/07-…`.
2. [ ] Responder las preguntas 🔴 de `docs/09-preguntas-abiertas.md`.
3. [x] Decidir la licencia → **GPL-3.0-or-later** (ADR-0003). _Solo falta confirmar que el docente no exige otra cosa._
4. [x] Resolver el nombre `Cargo.toml` (hecho). Siguiente: activar fmt/clippy/test en local.
5. [x] Definir e implementar la primera versión 3D (ADR-0004). Siguiente: jugarla, capturar evidencia y ajustar con la rúbrica.
