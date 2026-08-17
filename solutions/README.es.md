# Soluciones ejecutables

[English](README.md) | [Español](README.es.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

Esta crate reúne implementaciones de referencia para los ejercicios, las
catorce katas y los nueve proyectos de *Rust a fondo*.

Cada implementación lleva un marcador estable como este:

```rust
// SOLUTION: C24-E06
```

El enunciado completo, la discusión y las alternativas permanecen en el
libro. El repositorio aporta el código que se puede compilar, ejecutar y
modificar.

## Mapa de módulos

| Archivo | Capítulos |
|---|---:|
| `fundamentals.rs` | 1–10 |
| `functional.rs` | 11–14 |
| `abstraction.rs` | 15–20 |
| `memory.rs` | 21–24 |
| `organization.rs` | 25–29, 49–51 y 53–55 |
| `concurrency.rs` | 30–32 |
| `async_rust.rs` | 33–43 |
| `unsafe_low_level.rs` | 44–48 |
| `compiler.rs` | 52 |
| `katas.rs` | 56 |
| `projects.rs` | 57 |
| `mastery.rs` | 58 |

## Ejecutar las pruebas

Desde la raíz del repositorio:

```bash
cargo test -p course-solutions --locked
cargo test -p course-solutions --all-features --locked
cargo clippy -p course-solutions --all-targets --all-features --locked -- -D warnings
```

Para localizar o ejecutar una solución concreta:

```bash
rg "SOLUTION: C35-E04" solutions/src
cargo test -p course-solutions c35
```

Las soluciones abiertas representan una alternativa razonada, no la única
arquitectura válida.
