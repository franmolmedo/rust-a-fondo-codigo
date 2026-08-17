# Código de *Rust a fondo*

[English](README.md) | [Español](README.es.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

[![Verificación](https://github.com/franmolmedo/rust-a-fondo-codigo/actions/workflows/ci.yml/badge.svg)](https://github.com/franmolmedo/rust-a-fondo-codigo/actions/workflows/ci.yml)

Repositorio complementario de *Rust a fondo: Sin atajos: domina ownership,
concurrencia, async, unsafe y el diseño de sistemas robustos*.

Autor: **Francisco M. Olmedo Bueno**.

Contiene los ejemplos, ejercicios implementados, katas, proyectos y pruebas
automatizadas del libro. El manuscrito, el PDF y el EPUB no forman parte de
este repositorio.

## Qué incluye

- **891 bloques de código** conservados en `listings/`, identificados por capítulo.
- **403 soluciones ejecutables** de referencia en `solutions/`.
- **447 tests** asociados a las soluciones.
- Doctests, ejemplos `compile_fail` y casos `should_panic`.
- Laboratorios reales de macros procedurales, MIR, LLVM IR y assembly.
- Un manifiesto con trazabilidad y hashes SHA-256 de cada listado.

Los identificadores del libro son estables. Por ejemplo, `C24-E06` significa
«capítulo 24, ejercicio 6» y se conserva igual en todas las traducciones.

## Requisitos

- [Rustup](https://rustup.rs/) y Cargo.
- Python 3.11 o posterior.
- PowerShell 5.1 o posterior para `verify.ps1` en Windows.

El archivo `rust-toolchain.toml` instala automáticamente Rust 1.95.0 con
Clippy y rustfmt. Las crates declaran Rust 1.85 como versión mínima.

## Empezar

```bash
git clone https://github.com/franmolmedo/rust-a-fondo-codigo.git
cd rust-a-fondo-codigo
cargo test --workspace --all-targets --all-features --locked
```

Para ejecutar la misma auditoría que utiliza la integración continua:

```powershell
# Windows
.\verify.ps1
```

```bash
# Linux y macOS
./verify.sh
```

La auditoría comprueba hashes, TOML, configuraciones de features, tests,
doctests, formato, Clippy y la emisión real de MIR, LLVM IR y assembly.

## Declaración sobre el uso de inteligencia artificial

En el desarrollo de este repositorio se utilizaron herramientas de IA
generativa para crear y revisar partes del código, ejemplos, soluciones,
tests y documentación. Francisco M. Olmedo Bueno dirigió el trabajo, tomó las
decisiones finales y asume la responsabilidad sobre el material publicado.
La auditoría reproducible aporta evidencia técnica, pero no garantiza que el
código esté libre de errores ni que sea adecuado para cualquier uso concreto.

## Encontrar una solución

Busca el identificador que aparece en el libro:

```bash
rg "SOLUTION: C24-E06" solutions/src
```

También puedes ejecutar únicamente las pruebas de un capítulo o módulo:

```bash
cargo test -p course-solutions c24
cargo test -p course-solutions katas
cargo test -p course-solutions projects
```

El índice técnico de módulos está en
[`solutions/README.es.md`](solutions/README.es.md).

## Política de idioma

Los identificadores del código —módulos, API pública, tipos, campos, variables
y tests— están escritos en inglés para que el mismo código acompañe a todas
las traducciones del libro. Los identificadores estables como `C24-E06` nunca
se traducen. Los comentarios y mensajes en lenguaje natural de `listings/`
pueden reflejar la edición original en español porque esa carpeta conserva
literalmente los bloques publicados.

## Estructura

```text
.
├── solutions/         soluciones, katas y proyectos con tests
├── listings/          un archivo por bloque publicado en el libro
├── doctests/          harness de documentación y compile_fail
├── macro_lab/         macro procedural
├── macro_api/         API pública y reexportaciones de la macro
├── macro_fixture/     consumidor con dependencia renombrada
├── compiler_lab/      fuente estable para inspeccionar el compilador
├── tools/             auditoría reproducible
├── manifest.json      trazabilidad de los listados
└── VERIFICATION.md    último informe de verificación
```

`listings/`, `doctests/book.md` y `manifest.json` reflejan literalmente la
edición publicada. No deben modificarse de forma aislada: una errata debe
corregirse primero en el libro y propagarse después al corpus.

## Informar de una errata

Abre una incidencia indicando el identificador del bloque o solución, la
versión de `rustc`, el comportamiento observado y el esperado. Antes de enviar
un cambio, consulta [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Licencia

El código de este repositorio se distribuye bajo la [licencia MIT](LICENSE).
El manuscrito y las ediciones del libro no quedan cubiertos por esta licencia.

Copyright © 2026 Francisco M. Olmedo Bueno.
