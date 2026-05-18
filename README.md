# workshop-rust-study

"Command-Line Rust" (Ken Youens-Clark 著) の輪講リポジトリです。

## 構成

章ごとに独立した Cargo プロジェクトです。各コマンドはプロジェクトのディレクトリ内で実行してください。

```
workshop-rust-study/
├── chapter1/
│   └── hello/
│       ├── src/
│       └── tests/
├── chapter2/
│   ├── docs/
│   └── echor/
│       ├── src/
│       └── tests/
```

| ディレクトリ | 内容 |
|---|---|
| `chapter1/hello/` | hello world、`true` / `false` コマンド |
| `chapter2/echor/` | `echo` コマンドの実装（clap 2.33） |

各章の `docs/notes.md` に学習メモを残しています。

## Claude Code について

このリポジトリでは学習補助として Claude Code を使用しています。`CLAUDE.md` に動作指針を記載しています（コードを代わりに書かず、説明・調査・コマンド実行に徹する）。
