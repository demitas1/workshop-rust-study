# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a chapter-by-chapter Rust study repository, following the "Command-Line Rust" book. Each chapter is an independent Cargo project (no workspace). Work is done inside each chapter's subdirectory.

## IMPORTANT

このプロジェクトはRust学習のためのリポジトリです。
ユーザーがみずからコードを書いて理解することが目的のため、ユーザーに明示的に指示された場合をのぞき、コードをかわりに書いてはいけません。

### DO

- リポジトリ内のディレクトリやソースコードにアクセスし、ユーザーの書いたコードを理解すること
- cargo run, cargo build, cargo test の実行をおこない、その出力を解析すること
- cargo run, cargo build実行時のエラーの意味やコマンドの操作方法についてユーザーの質問に答えること
- Rust関係の用語やソフトウェア関連の用語について説明すること
- Rust公式ドキュメント、ソースコードにアクセスし、その内容を解説すること

### DO NOT

- ユーザーの指示なしにソースコードやディレクトリ構成のまちがいを指摘すること
- ユーザーの指示なしにソースコードを生成してプログラムを作成したり改良したりすること
- ユーザーの指示なしに cargo run, cargo build実行時のエラーの修正方法を提案すること

## Commands

All commands must be run from within a chapter's project directory (e.g., `chapter2/echor/`).

```bash
cargo build              # build
cargo run                # run the default binary
cargo run --bin <name>   # run a specific binary (e.g., true, false)
cargo run -- <args>      # pass arguments to the binary
cargo test               # run all tests
cargo test <test_name>   # run a single test by name
```

## Project Structure

Each chapter directory contains one Cargo project:

- `chapter1/hello/` — Hello world; also implements `true` and `false` as extra binaries. Uses `assert_cmd` for CLI integration tests in `tests/cli.rs`.
- `chapter2/echor/` — `echo` clone using `clap` 2.33 for argument parsing. Accepts one or more TEXT arguments and an optional `-n` flag to suppress the trailing newline.
  - `chapter2/docs/notes.md` — 学習メモ（ドキュメント参照方法、cat -A による改行確認、print!/println! の違い、Box<dyn Error>、TestResult パターン）

New chapters follow the same pattern: a single Cargo project per chapter, with integration tests in `tests/cli.rs` using `assert_cmd`.

## Testing Approach

Integration tests live in `tests/cli.rs` and use the `assert_cmd` crate to invoke the compiled binary and assert on stdout, stderr, and exit codes. Unit tests (if any) go directly in `src/main.rs` under `#[cfg(test)]`.
