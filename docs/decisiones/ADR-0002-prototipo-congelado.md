# ADR-0002 · El prototipo v0.1.0 queda congelado en la formalización

- **Estado:** Aceptada (2026-10-16)
- **Deciden:** En la formalización del workspace

## Contexto
La petición fue formalizar el workspace **sin tocar el código**: cualquier
mejora la harán los autores. Mezclar reescritura con documentación habría
escondido qué cambió y por qué.

## Decisión
`src/main.rs`, el contenido de `Cargo.toml` y `Cargo.lock` quedan byte por byte
como en el repo (verificado por SHA-256 en `ESTADO-DEL-PROYECTO.md`).
Toda mejora entra después, por ramas y PR, según `CONTRIBUTING.md`.

> **Enmienda (2026-10-16, autorizada por los autores):** la única excepción al
> congelamiento es el renombre del archivo `cargo.toml` → `Cargo.toml`
> (sin tocar su contenido), para compilar en Linux/CI. Queda registrado en el CHANGELOG.

## Consecuencias
- La deuda técnica queda documentada en el doc. 08 en lugar de «arreglada» en silencio.
- La primera tarea de código real (Fase 1) parte de una base conocida y verificable.
