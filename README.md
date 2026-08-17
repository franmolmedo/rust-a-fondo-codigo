# Code for *Rust a fondo*

[English](README.md) | [Español](README.es.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

[![Verification](https://github.com/franmolmedo/rust-a-fondo-codigo/actions/workflows/ci.yml/badge.svg)](https://github.com/franmolmedo/rust-a-fondo-codigo/actions/workflows/ci.yml)

Companion repository for *Rust a fondo: Sin atajos: domina ownership,
concurrencia, async, unsafe y el diseño de sistemas robustos*.

Author: **Francisco M. Olmedo Bueno**.

It contains the book's examples, implemented exercises, katas, projects, and
automated tests. The manuscript, PDF, and EPUB are not part of this repository.

## What's included

- **891 code blocks** preserved under `listings/` and identified by chapter.
- **403 executable reference solutions** under `solutions/`.
- **447 tests** associated with those solutions.
- Doctests, `compile_fail` examples, and `should_panic` cases.
- Real laboratories for procedural macros, MIR, LLVM IR, and assembly.
- A manifest providing traceability and SHA-256 hashes for every listing.

Book identifiers are stable. For example, `C24-E06` means “chapter 24,
exercise 6” and remains unchanged in every translation.

## Requirements

- [Rustup](https://rustup.rs/) and Cargo.
- Python 3.11 or later.
- PowerShell 5.1 or later for `verify.ps1` on Windows.

`rust-toolchain.toml` automatically installs Rust 1.95.0 with Clippy and
rustfmt. The crates declare Rust 1.85 as their minimum supported version.

## Getting started

```bash
git clone https://github.com/franmolmedo/rust-a-fondo-codigo.git
cd rust-a-fondo-codigo
cargo test --workspace --all-targets --all-features --locked
```

To run the same audit used by continuous integration:

```powershell
# Windows
.\verify.ps1
```

```bash
# Linux and macOS
./verify.sh
```

The audit checks hashes, TOML files, feature configurations, tests, doctests,
formatting, Clippy, and the actual emission of MIR, LLVM IR, and assembly.

## Artificial intelligence disclosure

Generative AI tools were used to create and review portions of the code,
examples, solutions, tests, and documentation in this repository. Francisco
M. Olmedo Bueno directed the work, made the final decisions, and assumes
responsibility for the published material. The reproducible audit provides
technical evidence, but it does not guarantee that the code is error-free or
suitable for any particular purpose.

## Finding a solution

Search for the identifier shown in the book:

```bash
rg "SOLUTION: C24-E06" solutions/src
```

You can also run the tests for a single chapter or module:

```bash
cargo test -p course-solutions c24
cargo test -p course-solutions katas
cargo test -p course-solutions projects
```

The technical module index is available in
[`solutions/README.md`](solutions/README.md).

## Language policy

Source identifiers—including modules, public APIs, types, fields, variables,
and tests—are written in English so the same code can accompany every
translation of the book. Stable identifiers such as `C24-E06` are never
translated. Natural-language comments and messages inside `listings/` may
mirror the original Spanish edition because that directory preserves the
published code blocks verbatim.

## Structure

```text
.
├── solutions/         solutions, katas, and projects with tests
├── listings/          one file for each code block published in the book
├── doctests/          documentation harness and compile_fail cases
├── macro_lab/         procedural macro implementation
├── macro_api/         public API and macro re-exports
├── macro_fixture/     consumer with a renamed dependency
├── compiler_lab/      stable source for compiler inspection
├── tools/             reproducible audit tooling
├── manifest.json      listing traceability
└── VERIFICATION.md    latest verification report
```

`listings/`, `doctests/book.md`, and `manifest.json` reproduce the published
edition. Do not edit them in isolation: fix an erratum in the book first and
then propagate it to the corpus.

## Reporting an erratum

Open an issue and include the code block or solution identifier, the `rustc`
version, the observed behavior, and the expected behavior. Before submitting
a change, read [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

The code in this repository is distributed under the [MIT License](LICENSE).
The manuscript and published editions of the book are not covered by this
license.

Copyright © 2026 Francisco M. Olmedo Bueno.
