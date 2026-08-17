# 『Rust a fondo』のコード

[English](README.md) | [Español](README.es.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

[![検証](https://github.com/franmolmedo/rust-a-fondo-codigo/actions/workflows/ci.yml/badge.svg)](https://github.com/franmolmedo/rust-a-fondo-codigo/actions/workflows/ci.yml)

『*Rust a fondo: Sin atajos: domina ownership, concurrencia, async, unsafe y
el diseño de sistemas robustos*』の付属リポジトリです。

著者: **Francisco M. Olmedo Bueno**

本書のサンプル、実装済み演習、kata、プロジェクト、自動テストを
収録しています。原稿、PDF、EPUB はこのリポジトリには含まれません。

## 収録内容

- `listings/` に章ごとに整理された **891 個のコードブロック**。
- `solutions/` に収録された **403 個の実行可能な参照解答**。
- 解答に対応する **447 個のテスト**。
- doctest、`compile_fail` サンプル、`should_panic` ケース。
- 手続きマクロ、MIR、LLVM IR、assembly の実動ラボ。
- 各 listing の追跡情報と SHA-256 ハッシュを保持する manifest。

本書の識別子は固定です。たとえば `C24-E06` は「第24章・演習6」を
表し、どの翻訳版でも変更されません。

## 必要な環境

- [Rustup](https://rustup.rs/) と Cargo。
- Python 3.11 以降。
- Windows で `verify.ps1` を実行する場合は PowerShell 5.1 以降。

`rust-toolchain.toml` により、Clippy と rustfmt を含む Rust 1.95.0 が
自動的にインストールされます。各 crate の MSRV は Rust 1.85 です。

## はじめに

```bash
git clone https://github.com/franmolmedo/rust-a-fondo-codigo.git
cd rust-a-fondo-codigo
cargo test --workspace --all-targets --all-features --locked
```

継続的インテグレーションと同じ監査を実行するには、次を使います。

```powershell
# Windows
.\verify.ps1
```

```bash
# Linux と macOS
./verify.sh
```

この監査では、ハッシュ、TOML、feature 構成、テスト、doctest、
フォーマット、Clippy、および MIR・LLVM IR・assembly の実際の生成を
確認します。

## 人工知能の利用について

このリポジトリのコード、サンプル、解答、テスト、ドキュメントの
一部の作成およびレビューに生成 AI ツールを使用しました。
Francisco M. Olmedo Bueno が作業を指揮し、最終判断を行い、公開物に
責任を負います。再現可能な監査は技術的な根拠を提供しますが、
コードに誤りがないこと、または特定用途に適することを保証しません。

## 解答を探す

本書に記載された識別子を検索します。

```bash
rg "SOLUTION: C24-E06" solutions/src
```

特定の章またはモジュールだけをテストすることもできます。

```bash
cargo test -p course-solutions c24
cargo test -p course-solutions katas
cargo test -p course-solutions projects
```

モジュールの技術索引は
[`solutions/README.ja.md`](solutions/README.ja.md) にあります。

## 言語方針

モジュール、公開 API、型、フィールド、変数、テストを含むソースコードの
識別子は、すべての翻訳版で同じコードを利用できるよう英語で記述します。
`C24-E06` のような固定識別子は翻訳しません。`listings/` は出版された
コードブロックをそのまま保存するため、自然言語のコメントやメッセージが
スペイン語原版と同じ場合があります。

## 構成

```text
.
├── solutions/         テスト付きの解答、kata、プロジェクト
├── listings/          本書に掲載したコードブロックごとのファイル
├── doctests/          ドキュメント用 harness と compile_fail ケース
├── macro_lab/         手続きマクロの実装
├── macro_api/         公開 API とマクロの再エクスポート
├── macro_fixture/     依存関係を別名にした利用側 crate
├── compiler_lab/      コンパイラ調査用の安定したソース
├── tools/             再現可能な監査ツール
├── manifest.json      listing の追跡情報
└── VERIFICATION.md    最新の検証レポート
```

`listings/`、`doctests/book.md`、`manifest.json` は出版版を再現します。
これらを単独で編集しないでください。正誤修正はまず本書に反映し、その後
corpus に同期します。

## 正誤を報告する

issue を作成し、コードブロックまたは解答の識別子、`rustc` のバージョン、
実際の動作、期待する動作を記載してください。変更を送る前に
[`CONTRIBUTING.md`](CONTRIBUTING.md) を確認してください。

## ライセンス

このリポジトリのコードは [MIT License](LICENSE) で配布されます。
本書の原稿および出版版は、このライセンスの対象外です。

Copyright © 2026 Francisco M. Olmedo Bueno.
