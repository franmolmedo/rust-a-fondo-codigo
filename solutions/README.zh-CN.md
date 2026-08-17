# 可执行解答

[English](README.md) | [Español](README.es.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

此 crate 收录《Rust a fondo》中各项练习、14 个 kata 和 9 个项目的
参考实现。

每个实现都带有如下稳定标记：

```rust
// SOLUTION: C24-E06
```

完整题目、讨论和其他方案仍保留在书中。本仓库提供可供编译、运行和修改
的代码。

## 模块索引

| 文件 | 章节 |
|---|---:|
| `fundamentals.rs` | 1–10 |
| `functional.rs` | 11–14 |
| `abstraction.rs` | 15–20 |
| `memory.rs` | 21–24 |
| `organization.rs` | 25–29、49–51 和 53–55 |
| `concurrency.rs` | 30–32 |
| `async_rust.rs` | 33–43 |
| `unsafe_low_level.rs` | 44–48 |
| `compiler.rs` | 52 |
| `katas.rs` | 56 |
| `projects.rs` | 57 |
| `mastery.rs` | 58 |

## 运行测试

在仓库根目录执行：

```bash
cargo test -p course-solutions --locked
cargo test -p course-solutions --all-features --locked
cargo clippy -p course-solutions --all-targets --all-features --locked -- -D warnings
```

查找或运行某个具体解答：

```bash
rg "SOLUTION: C35-E04" solutions/src
cargo test -p course-solutions c35
```

开放式题目的参考解答只代表一种有充分理由的方案，并非唯一有效的
架构设计。
