# 実行可能な解答

[English](README.md) | [Español](README.es.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

この crate には、『Rust a fondo』の演習、14 個の kata、9 個の
プロジェクトに対する参照実装が収録されています。

各実装には次のような固定マーカーがあります。

```rust
// SOLUTION: C24-E06
```

問題文全体、解説、代替案は本書にあります。このリポジトリでは、
コンパイル、実行、変更が可能なコードを提供します。

## モジュール一覧

| ファイル | 章 |
|---|---:|
| `fundamentals.rs` | 1–10 |
| `functional.rs` | 11–14 |
| `abstraction.rs` | 15–20 |
| `memory.rs` | 21–24 |
| `organization.rs` | 25–29、49–51、53–55 |
| `concurrency.rs` | 30–32 |
| `async_rust.rs` | 33–43 |
| `unsafe_low_level.rs` | 44–48 |
| `compiler.rs` | 52 |
| `katas.rs` | 56 |
| `projects.rs` | 57 |
| `mastery.rs` | 58 |

## テストの実行

リポジトリのルートから実行します。

```bash
cargo test -p course-solutions --locked
cargo test -p course-solutions --all-features --locked
cargo clippy -p course-solutions --all-targets --all-features --locked -- -D warnings
```

特定の解答を検索または実行するには、次を使います。

```bash
rg "SOLUTION: C35-E04" solutions/src
cargo test -p course-solutions c35
```

自由回答形式の解答は、唯一の正しいアーキテクチャではなく、根拠のある
選択肢の一つを示します。
