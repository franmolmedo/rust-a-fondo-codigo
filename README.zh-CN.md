# 《Rust a fondo》配套代码

[English](README.md) | [Español](README.es.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

[![验证状态](https://github.com/franmolmedo/rust-a-fondo-codigo/actions/workflows/ci.yml/badge.svg)](https://github.com/franmolmedo/rust-a-fondo-codigo/actions/workflows/ci.yml)

本仓库是《*Rust a fondo: Sin atajos: domina ownership, concurrencia, async,
unsafe y el diseño de sistemas robustos*》的配套代码仓库。

作者：**Francisco M. Olmedo Bueno**。

仓库包含书中的示例、已实现练习、kata、项目和自动化测试，不包含书稿、
PDF 或 EPUB。

## 仓库内容

- `listings/` 中按章节保存的 **891 个代码块**。
- `solutions/` 中的 **403 个可执行参考解答**。
- 与参考解答对应的 **447 个测试**。
- doctest、`compile_fail` 示例和 `should_panic` 用例。
- 可实际运行的过程宏、MIR、LLVM IR 和 assembly 实验。
- 为每个 listing 提供追踪信息及 SHA-256 哈希的 manifest。

书中的标识符保持稳定。例如，`C24-E06` 表示“第24章练习6”，在所有
翻译版本中都不会改变。

## 环境要求

- [Rustup](https://rustup.rs/) 和 Cargo。
- Python 3.11 或更高版本。
- Windows 上运行 `verify.ps1` 需要 PowerShell 5.1 或更高版本。

`rust-toolchain.toml` 会自动安装包含 Clippy 和 rustfmt 的 Rust 1.95.0。
各 crate 声明的最低支持版本为 Rust 1.85。

## 快速开始

```bash
git clone https://github.com/franmolmedo/rust-a-fondo-codigo.git
cd rust-a-fondo-codigo
cargo test --workspace --all-targets --all-features --locked
```

如需运行与持续集成相同的审计流程：

```powershell
# Windows
.\verify.ps1
```

```bash
# Linux 和 macOS
./verify.sh
```

该流程会检查哈希、TOML、feature 配置、测试、doctest、格式、Clippy，
以及 MIR、LLVM IR 和 assembly 的实际生成结果。

## 人工智能使用声明

本仓库的部分代码、示例、解答、测试和文档在创建及审阅过程中使用了
生成式 AI 工具。Francisco M. Olmedo Bueno 负责指导整个过程、作出最终
决定，并对发布内容承担责任。可复现审计能够提供技术证据，但不能保证
代码完全没有错误，也不能保证其适用于任何特定用途。

## 查找解答

使用书中出现的标识符进行搜索：

```bash
rg "SOLUTION: C24-E06" solutions/src
```

也可以只运行某一章或模块的测试：

```bash
cargo test -p course-solutions c24
cargo test -p course-solutions katas
cargo test -p course-solutions projects
```

模块技术索引位于
[`solutions/README.zh-CN.md`](solutions/README.zh-CN.md)。

## 语言规范

模块、公共 API、类型、字段、变量和测试等源代码标识符均使用英语，
从而让同一份代码能够配合本书的所有翻译版本使用。`C24-E06` 等稳定
标识符不会被翻译。由于 `listings/` 会逐字保存已出版的代码块，其中的
自然语言注释和消息可能与西班牙语原版保持一致。

## 目录结构

```text
.
├── solutions/         带测试的解答、kata 和项目
├── listings/          书中每个代码块对应一个文件
├── doctests/          文档测试及 compile_fail 用例
├── macro_lab/         过程宏实现
├── macro_api/         公共 API 和宏重新导出
├── macro_fixture/     使用重命名依赖项的消费方 crate
├── compiler_lab/      用于编译器分析的稳定源码
├── tools/             可复现审计工具
├── manifest.json      listing 追踪信息
└── VERIFICATION.md    最新验证报告
```

`listings/`、`doctests/book.md` 和 `manifest.json` 与出版版本保持一致。
请勿单独修改这些文件：应先在书中修正勘误，再将修改同步到代码 corpus。

## 报告勘误

请创建 issue，并注明代码块或解答标识符、`rustc` 版本、实际行为和预期
行为。提交修改前，请先阅读 [`CONTRIBUTING.md`](CONTRIBUTING.md)。

## 许可证

本仓库中的代码依据 [MIT License](LICENSE) 发布。书稿及已出版版本不在
该许可证的适用范围内。

Copyright © 2026 Francisco M. Olmedo Bueno.
