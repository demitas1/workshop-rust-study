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

## ドキュメントから型とメソッドを調べる

### matches の型を調べる

`src/main.rs` の `let matches = App::new(...).get_matches()` で型を明示する場合、`get_matches()` の戻り値型をドキュメントで確認する。

`https://docs.rs/clap/2.33.0/clap/struct.App.html#method.get_matches`

シグネチャに `fn get_matches(self) -> ArgMatches<'a>` と書かれており、戻り値型が `ArgMatches<'a>` とわかる。

**補足: 型を明示する場合**
`use clap::ArgMatches;` を追加した上で以下のどちらかで書ける。

```rust
let matches: ArgMatches = ...
let matches: ArgMatches<'static> = ...
```

`'a` はライフタイムの名前で宣言が必要なため `main` 関数では使えない。`ArgMatches` と書いた場合はコンパイラが推論する。`'static` はプログラム全体にわたって有効なライフタイムを表す特別な名前で、宣言なしで使える。

### メソッドを探して values_of_lossy を見つける

`ArgMatches` のドキュメントページのメソッド一覧から `values_of_lossy` を見つけられる。複数の引数値を取得するメソッドで、戻り値型は `Option<Vec<String>>`。

### メソッドの機能は src を読む

ドキュメントの説明が薄い場合、各メソッドの右端にある `[src]` リンクからソースコードを直接読む。実装を読むことで動作を正確に把握できる。

`values_of_lossy` のソースを読むと `.to_string_lossy().into_owned()` を呼んでいることがわかる。`into_owned()` が `Cow` を `String` に変換しているため、戻り値は `Cow` ではなく `Vec<String>` になっている。

### 補足: value_of_lossy と Cow

`value_of_lossy`（単数形）の戻り値は `Option<Cow<'a, str>>`。`Cow` は Copy on Write の略。

内部では `to_string_lossy()` を呼んでおり、引数の UTF-8 の状態によって返す値が分岐する。

| 引数の状態 | 返る Cow の種類 | 内容 |
|---|---|---|
| 有効な UTF-8 | `Cow::Borrowed(&str)` | 元データへの参照。コピーなし |
| 無効バイトを含む | `Cow::Owned(String)` | 無効バイトを置換文字（`\u{FFFD}`）で埋めた新しい String |

どちらのケースも呼び出し側は同じ型として扱える点が `Cow` の利点。

`values_of_lossy`（複数形）は `.into_owned()` で `Cow` を `String` に統一してから `Vec` に集めるため、戻り値から `Cow` がなくなっている。`Borrowed` の場合も `into_owned()` で新たに `String` を確保する。

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

## Box<dyn std::error::Error>

`type TestResult = Result<(), Box<dyn std::error::Error>>` の各部分の意味。

- `Result<(), E>` — 成功か失敗かを表す型。`()` は成功時に返す値がないことを示す
- `std::error::Error` — 標準ライブラリが定義するエラー型のトレイト（インターフェース）
- `dyn std::error::Error` — 「このトレイトを実装した何らかの型」。具体的な型名を指定しない
- `Box<...>` — `dyn` を使う型はコンパイル時にサイズが不定なためスタックに置けない。`Box` でヒープに置き、ポインタをスタックで持つ

```
スタック         ヒープ
┌─────────┐     ┌──────────────────┐
│ pointer ├────►│ 実際のデータ      │
└─────────┘     └──────────────────┘
```

`Box<dyn std::error::Error>` にすることで、異なる種類のエラー型を `?` 演算子でまとめて返せる。

## テストにおける2種類のエラー

`dies_no_args` を例に、2種類のエラーを区別する。

```rust
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?   // ② テストインフラのエラー
        .assert()
        .failure()                 // ① プログラムの終了コードの検証
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}
```

**① プログラムの終了コード（テストが検証したいもの）**
引数なしで `echor` を実行したとき、exit code が 0 以外になり stderr に "USAGE" が含まれることを `.assert()` で検証する。これはテスト対象プログラムの振る舞いの確認。

**② テストインフラのエラー**
`Command::cargo_bin("echor")` でバイナリが見つからないなど、テスト実行環境側の失敗。`?` で `Err` を返す。`TestResult` の `Box<dyn std::error::Error>` はこちらのために使われており、①とは無関係。

`.assert().failure()` の検証が失敗した場合は `Result::Err` ではなく `panic` になる。

**補足: `?` を `unwrap()` に変えた場合**
どちらもテストインフラ側のエラーでテストを失敗させる点は同じだが、挙動が異なる。

| | `?` | `unwrap()` |
|---|---|---|
| 失敗時の挙動 | `Err(e)` を返す | `panic!` する |
| 返り値型 | `-> TestResult` が必要 | `-> ()` または省略でよい |
| エラーメッセージ | テストフレームワークが整形して表示 | panic のスタックトレースが表示 |

`runs()` や `hello1()` が `unwrap()` を使い返り値なしで書かれているのがその例。

## 引数がない場合の main の動作

`get_matches()` が引数を解析する時点で clap が処理を止める。

```rust
let matches = App::new("echor")
    ...
    .arg(
        Arg::with_name("text")
            .required(true)   // ← 必須引数
            .min_values(1),
    )
    .get_matches();           // ← 引数がなければここでプロセス終了
```

`required(true)` の引数がない場合、clap は内部で `std::process::exit(1)` を呼び出し、その前に stderr へエラーメッセージと USAGE を出力する。`std::process::exit()` はその時点でプロセスを即座に終了させるため、以降の行は実行されない。

## print! と println! の違い

| マクロ | 末尾の改行 |
|--------|-----------|
| `println!` | 自動で `\n` を追加する |
| `print!`   | `\n` を追加しない |

`println!("{}", text)` と `print!("{}\n", text)` は同じ出力になる。
文字列に `\n` を含めつつ `println!` を使うと改行が二重になるので注意。
