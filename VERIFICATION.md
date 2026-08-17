# Verificación del corpus

**Resultado: APROBADA.**

- Toolchain: `rustc 1.95.0 (59807616e 2026-04-14)`
- Bloques conservados: **891**
- Bloques Rust verificados por Cargo: **645/759**
- Rust ejecutable: **541**
- Rust de solo compilación: **20**
- Errores esperados: **82**
- Panics esperados: **2**
- Fragmentos ilustrativos: **32**
- Fragmentos contextuales: **82**
- TOML válido: **25/25**
- Soluciones ejecutables de referencia: **403**
- Tests declarados en la crate de soluciones: **447**

## Comprobaciones

- **OK** — `cargo check --workspace --locked` (0.27 s)
- **OK** — `cargo check --workspace --all-features --locked` (0.14 s)
- **OK** — `cargo test --workspace --all-targets --locked` (0.89 s)
- **OK** — `cargo test --workspace --all-targets --all-features --locked` (0.88 s)
- **OK** — `cargo test --workspace --doc --locked` (5.67 s)
- **OK** — `cargo test --workspace --doc --all-features --locked` (6.11 s)
- **OK** — `cargo fmt --all --check` (0.2 s)
- **OK** — `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` (0.31 s)
- **OK** — `python tools/compiler_probe.py` (0.48 s)
