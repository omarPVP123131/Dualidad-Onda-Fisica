# Cómo contribuir (Omar + Leonardo)

Este proyecto lo desarrollan dos personas. Esta guía es corta a propósito:
su trabajo es que nunca se pisen, nunca se pierda código y cualquier profesor
pueda seguir qué hizo cada quién.

## Flujo de trabajo

1. **Nada directo a `main`.** `main` debe compilar siempre.
2. Una rama por tarea: `feat/…`, `fix/…`, `docs/…`, `refactor/…`
   (ej.: `feat/escena-3d-camara`).
3. Commits pequeños con mensaje [Conventional Commits](https://www.conventionalcommits.org/es/):
   `feat: …`, `fix: …`, `docs: …`, `refactor: …`, `test: …`.
4. Pull Request hacia `main` usando la plantilla de `.github/`. Lo revisa
   **la otra persona**, no quien lo escribió.
5. Antes de pedir revisión, en su máquina debe pasar:

```bash
Cargo fmt --check
Cargo clippy -- -D warnings
Cargo test
Cargo run --release   # y probar los controles a mano
```

> Si `Cargo` no encuentra el proyecto en Linux, están en un clon viejo:
> ver la nota del nombre `Cargo.toml` en `docs/04-instalacion-y-desarrollo.md`.

## Reglas de oro

- **La física no se cambia “de pasada”.** Todo cambio en las fórmulas se
  explica en el PR, se referencia en `docs/02-fisica.md` y se actualiza el doc.
- **Primero decidir, luego codear.** Toda decisión importante (3D, librerías,
  licencia) nace como ADR en `docs/decisiones/` antes del código.
- **Documentar al mismo tiempo que el código**, no “al final” — al final nunca llega.
- Si algo quedó provisional, se anota en `docs/08-riesgos-y-deuda-tecnica.md`.

## Reparto sugerido de revisión

| Área | Autor principal | Revisor |
|---|---|---|
| Física y validación | por definir | la otra persona |
| Escena 3D y cámara | por definir | la otra persona |
| UI, textos y accesibilidad | por definir | la otra persona |
| Documentación e informe | por definir | la otra persona |

*Llenen la tabla en su primera reunión y déjenla registrada aquí con fecha.*
