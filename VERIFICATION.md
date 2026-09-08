# Verificación del corpus

**Resultado: APROBADA.**

- Toolchain: `rustc 1.95.0 (59807616e 2026-04-14)`
- Bloques conservados: **892**
- Bloques Rust verificados por Cargo: **650/760**
- Rust ejecutable: **546**
- Rust de solo compilación: **20**
- Errores esperados: **82**
- Panics esperados: **2**
- Fragmentos ilustrativos: **31**
- Fragmentos contextuales: **79**
- TOML válido: **25/25**
- Soluciones ejecutables de referencia: **403**
- Tests declarados en la crate de soluciones: **604**

## Comprobaciones

- **OK** — `cargo check --workspace --locked` (6.53 s)
- **OK** — `cargo check --workspace --all-features --locked` (1.77 s)
- **OK** — `cargo test --workspace --all-targets --locked` (20.92 s)
- **OK** — `cargo test --workspace --all-targets --all-features --locked` (17.59 s)
- **OK** — `cargo test --workspace --doc --locked` (7.55 s)
- **OK** — `cargo test --workspace --doc --all-features --locked` (9.12 s)
- **OK** — `cargo fmt --all --check` (0.33 s)
- **OK** — `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` (4.61 s)
- **OK** — `python tools/compiler_probe.py` (1.66 s)
- **OK** — `cargo doc --workspace --no-deps --all-features --locked` (2.64 s)
- **OK** — `cargo test --manifest-path fixtures/chapter26-workspace/Cargo.toml --workspace --locked` (1.27 s)
- **OK** — `cargo fmt --manifest-path fixtures/chapter26-workspace/Cargo.toml --all --check` (0.11 s)
- **OK** — `cargo clippy --manifest-path fixtures/chapter26-workspace/Cargo.toml --workspace --all-targets --locked -- -D warnings` (0.41 s)
- **OK** — `cargo +1.86.0 check --workspace --all-targets --all-features --locked` (10.7 s)
- **OK** — `cargo +1.86.0 test --workspace --all-targets --locked` (21.64 s)
- **OK** — `cargo +1.86.0 test --workspace --all-targets --all-features --locked` (14.06 s)
- **OK** — `cargo +1.86.0 test --workspace --doc --all-features --locked` (6.47 s)
