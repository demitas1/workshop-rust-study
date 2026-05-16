# Chapter 2 メモ

## クレートのドキュメントへのアクセス

### 直接 URL
`https://docs.rs/<クレート名>` でアクセスできる。

例: clap なら `https://docs.rs/clap`

標準ライブラリは docs.rs ではなく以下を参照する。

例: `https://doc.rust-lang.org/std/fs/fn.read_to_string.html`

### cargo doc --open
プロジェクトディレクトリで実行すると、依存クレートのドキュメントをローカル生成してブラウザで開く。
標準ライブラリは表示されない。

```bash
cargo doc --open
```

## cat -A で改行文字を確認する

`cat -A` を使うと、改行文字 `\n` の位置が `$` として表示される。
パイプと組み合わせてプログラムの出力を検査できる。

```bash
cargo run -- "Hello there" | cat -A
```

出力例：
```
Hello there$   # \n が1つ → 正常
$              # さらに $ がある場合は余分な \n が出力されている
```

## print! と println! の違い

| マクロ | 末尾の改行 |
|--------|-----------|
| `println!` | 自動で `\n` を追加する |
| `print!`   | `\n` を追加しない |

`println!("{}", text)` と `print!("{}\n", text)` は同じ出力になる。
文字列に `\n` を含めつつ `println!` を使うと改行が二重になるので注意。
