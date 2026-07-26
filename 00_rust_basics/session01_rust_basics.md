# 第1回 Rust勉強会 — C++経験者のためのRust基礎

## この資料について

- **対象**: C++に精通し、Rustは初めての開発者
- **目的**: 書籍『Rustの練習帳』（O'Reilly Japan, Ken Youens-Clark 著）を読み進めるための**最低限の基礎知識**を共有する
- **進め方**: 各セクションを「**ポイント → サンプルコード写経**」の順で進める。サンプルコードは原則として [Rust Playground](https://play.rust-lang.org/) にそのまま貼り付けて動作確認可能
- **重視する点**: **C++の直感がRustで通用しない箇所**に焦点を当てる（考え方の違い > 構文の違い）

## 目次

1. [開発ワークフロー（Cargo / rustc）](#1-開発ワークフローcargo--rustc)
2. [変数の不変性がデフォルト](#2-変数の不変性がデフォルトlet--let-mut)
3. [文と式（Rustは式指向言語）](#3-文と式rustは式指向言語)
4. [所有権 — Rust最大の特徴](#4-所有権ownership--rust最大の特徴)
5. [借用と参照](#5-借用と参照t--mut-t)
6. [`String` と `&str` / `Vec<T>` と `&[T]`](#6-string-と-str--vect-と-t)
7. [`Option<T>` — null が存在しない世界](#7-optiont--null-が存在しない世界)
8. [`Result<T, E>` — 例外のないエラー処理](#8-resultt-e--例外のないエラー処理)
9. [`enum` とパターンマッチング](#9-enum-とパターンマッチング)
10. [`struct` と `impl` と `trait`（継承はない）](#10-struct-と-impl-と-trait継承はない)
11. [マクロと書籍頻出キーワード](#11-マクロと書籍頻出キーワード)
- [付録](#付録)

---

## 1. 開発ワークフロー（Cargo / rustc）

### ポイント

- Rustはコンパイラ `rustc` と統合ツール `cargo` を持つ。通常は `cargo` だけ覚えればよい。
- プロジェクト作成 → ビルド → 実行 → テストが `cargo` の1コマンドで完結。
- 依存関係は `Cargo.toml` に書くだけ。ヘッダファイルもリンク設定も不要。
- ビルド成果物は `target/` 以下、依存クレートのソースは `~/.cargo/` にキャッシュされる。
- コードフォーマッタ `cargo fmt`、リンター `cargo clippy` が標準で付属。

> **C++では** CMake / Makefileなど複数のツールの組み合わせが必要。  
> **Rustでは** `Cargo.toml` 1ファイルにプロジェクト定義と依存関係が集約される。

### 主要コマンド

```bash
cargo new hello         # 新規プロジェクト作成
cargo build             # デバッグビルド
cargo run               # ビルド + 実行
cargo test              # テスト実行
cargo build --release   # 最適化ビルド
cargo check             # 型検査のみ（高速）
cargo fmt               # フォーマット
cargo clippy            # リンター
```

### サンプル（最小プロジェクト）

```bash
$ cargo new hello
$ cd hello
$ cargo run
   Compiling hello v0.1.0 (...)
    Finished dev [...] target(s) in 0.50s
     Running `target/debug/hello`
Hello, world!
```

生成される `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

生成される `Cargo.toml`:

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

[dependencies]
```

---

## 2. 変数の不変性がデフォルト（`let` / `let mut`）

### ポイント

- `let` で束縛した変数は**デフォルトで不変**（immutable）。再代入するとコンパイルエラー。
- 可変にしたければ明示的に `let mut` と書く。
- 型推論があるので型注釈は省略可。必要なら `let x: i32 = 1;` と書く。
- **シャドーイング**: 同じ名前で `let` を再度書くと新しい変数として束縛される。型変換にも便利。

> **C++では** `int x = 1;` は可変、不変にするには `const int x = 1;`  
> **Rustでは** `let x = 1;` は不変、可変にするには `let mut x = 1;` ← **真逆**

### サンプル（悪い例）

```rust
fn main() {
    let x = 1;
    x = 2;  // ← コンパイルエラー: cannot assign twice to immutable variable `x`
    println!("{}", x);
}
```

### サンプル（良い例）

```rust
fn main() {
    let mut x = 1;
    x = 2;                      // OK
    println!("{}", x);          // => 2

    // シャドーイング（型を変えることもできる）
    let s = "42";
    let s: i32 = s.parse().unwrap();
    println!("{}", s);          // => 42
}
```

---

## 3. 文と式（Rustは式指向言語）

### ポイント

- Rustの**ブロック `{ ... }` は値を返す**。最後の式がブロックの値になる（**末尾にセミコロン無し**）。
- セミコロンを付けると式が**文**になり、値は捨てられる。型エラーの原因になりやすい。
  - 式: 評価すると値を返すもの
  - 文: 評価しても値を返さないもの（副作用のためだけに書く。e.g. println!())
- `if`, `match`, `loop` はすべて**式**。なので `let y = if cond { 1 } else { 2 };` と書ける。
- 関数の戻り値も「本体ブロックの最後の式」。`return` は早期リターン専用で、末尾では省略するのが慣習。

> **C++では** `if` は文。三項演算子 `cond ? a : b` で代用が必要。  
> **Rustでは** `if` が式なのでそのまま代入できる。三項演算子は**存在しない**。

### サンプル

```rust
fn abs(x: i32) -> i32 {
    if x >= 0 { x } else { -x }  // ← セミコロンなし = 関数の戻り値
}

fn main() {
    // if式
    let n = 10;
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, parity);   // => 10 is even

    // ブロック式
    let y = {
        let t = 3;
        t * t + 1                       // ← セミコロン無し = ブロックの値
    };
    println!("{}", y);                  // => 10

    println!("{}", abs(-7));            // => 7
}
```

### セミコロンの落とし穴

```rust
fn bad() -> i32 {
    5;   // ← セミコロン付きは「式を捨てる文」。戻り値が () になりエラー
}
// error[E0308]: mismatched types — expected `i32`, found `()`
```

---

## 4. 所有権（ownership） — Rust最大の特徴

### ポイント

- Rustでは**すべての値に唯一の所有者**がいる。所有者のスコープを抜けると値は破棄される（`Drop`）。
- 代入・関数渡しでは**ムーブ**される（所有権の移動）。**元の変数は以降アクセス不可**。
- 整数・浮動小数・`bool` など固定サイズの型は `Copy` トレイトを実装しているので例外的にコピーされる。
- コピーしたければ明示的に `.clone()` を呼ぶ。**コピーにはコストがかかる**からこそ明示。

> **C++では** 代入は暗黙コピー、`std::move` で明示的にムーブ（しかもムーブ元の状態は有効。C++のstd::moveはただの右辺値へのキャスト）。  
> **Rustでは** 代入は**暗黙ムーブ**、コピーは明示的に `.clone()`。ムーブ後の元変数は**使用不可**。← **真逆**

### サンプル（悪い例）

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;             // ← s1 の所有権が s2 にムーブ
    println!("{}", s1);      // ← エラー: borrow of moved value: `s1`
}
```

コンパイルエラー:

```
error[E0382]: borrow of moved value: `s1`
```

### サンプル（良い例）

```rust
fn main() {
    // Copy 型（i32）はムーブされず、暗黙コピーされる
    let a = 10;
    let b = a;
    println!("{} {}", a, b);        // => 10 10

    // String は Copy ではないので、コピーしたければ clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);      // => hello hello
}
```

### スコープと Drop（RAIIの言語機能版）

```rust
fn main() {
    {
        let s = String::from("temp");
        println!("{}", s);
    }  // ← ここで s は drop される（C++のRAIIと同じだが、言語仕様で強制）
    // println!("{}", s);  // ← スコープ外なのでエラー
}
```

---

## 5. 借用と参照（`&T` / `&mut T`）

### ポイント

- ムーブしたくないときは**参照**を渡す = **借用**する。
- `&T` は**不変参照**（読み取り専用）。`&mut T` は**可変参照**（書き込み可）。
- **借用規則**: ある時点で1つの値に対して
  - **複数の不変参照** はOK、または
  - **ただ1つの可変参照** はOK。
  - **両者の混在は不可**（データ競合をコンパイル時に禁止）。
- 参照はスコープ外の値を指せない（**ダングリング参照はコンパイル時に拒否**）。

> **C++では** ポインタ/参照の有効期間はプログラマの責任。ダングリングは未定義動作（UB）。  
> **Rustでは** 借用チェッカーが参照の有効性を**コンパイル時**に検証。ダングリングは原理的に発生しない。

### サンプル

```rust
fn len(s: &String) -> usize {     // 不変借用
    s.len()
}  // ← s は借用なので drop されない

fn push_world(s: &mut String) {   // 可変借用
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    println!("{}", len(&s));      // => 5   （不変借用）
    push_world(&mut s);           //         （可変借用）
    println!("{}", s);            // => hello world
}
```

### 借用規則違反の例

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;   // ← エラー: cannot borrow `s` as mutable
                       //    because it is also borrowed as immutable
    println!("{} {}", r1, r2);
}
```

---

## 6. `String` と `&str` / `Vec<T>` と `&[T]`

### ポイント

- Rustの文字列・配列には**「所有型」と「借用型」のペア**がある。
  - `String` = 所有・ヒープ確保・可変長 / `&str` = 不変スライス（借用）
  - `Vec<T>` = 所有・ヒープ確保・可変長 / `&[T]` = 不変スライス（借用）
- 文字列リテラル `"hello"` は `&'static str` 型（プログラムのデータ領域を指す参照）。
- 関数の引数は、可能なかぎり**借用型**（`&str`, `&[T]`）で受けると汎用的。

> **C++では** `std::string` と `std::string_view`、`std::vector<T>` と `std::span<T>` の関係に相当。  
> **Rustでは** この区別が**言語の核**に組み込まれており、所有権ルールと密接に結びつく。

### サンプル

```rust
fn greet(name: &str) {                 // &str なら String も &str も渡せる
    println!("Hello, {}", name);
}

fn sum(xs: &[i32]) -> i32 {            // &[i32] なら Vec<i32> も配列も渡せる
    xs.iter().sum()
}

fn main() {
    let lit: &str = "world";                    // リテラルは &'static str
    let owned: String = String::from("Rust");   // 所有型

    greet(lit);          // => Hello, world
    greet(&owned);       // => Hello, Rust （String は &str に暗黙変換される）

    let v: Vec<i32> = vec![1, 2, 3];
    let arr: [i32; 3]  = [4, 5, 6];
    println!("{}", sum(&v));            // => 6
    println!("{}", sum(&arr));          // => 15
}
```

---

## 7. `Option<T>` — null が存在しない世界

### ポイント

- Rustに `null` / `nullptr` は**存在しない**。
- 「値があるかもしれない」は標準の `enum Option<T> { Some(T), None }` で表す。
- 使うには `match` / `if let` で**中身を取り出す必要がある** → ヌルチェック忘れが起きない。
- `unwrap()` で中身を取り出せるが、`None` のときは **panic**。確実に `Some` とわかっている場合のみ。
- `unwrap_or(default)`, `map(f)`, `and_then(f)` など豊富なコンビネータがある。

> **C++では** `std::optional<T>` / 生ポインタ / `nullptr` など複数の選択肢。  
> **Rustでは** 値の不在は `Option<T>` で表すのが**唯一の標準**。型システムで見落としを防ぐ。

### サンプル

```rust
fn first_word(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

fn main() {
    let a = first_word("hello world");
    let b = first_word("");

    // match: 網羅的に処理
    match a {
        Some(w) => println!("got: {}", w),     // => got: hello
        None    => println!("empty"),
    }

    // if let: 1パターンだけ関心があるとき
    if let Some(w) = b {
        println!("got: {}", w);
    } else {
        println!("empty");                      // => empty
    }

    // デフォルト値
    let w = first_word("").unwrap_or("<none>");
    println!("{}", w);                          // => <none>
}
```

---

## 8. `Result<T, E>` — 例外のないエラー処理

### ポイント

- Rustに**例外機構は存在しない**。
- 失敗しうる処理は `enum Result<T, E> { Ok(T), Err(E) }` を返す。
- 呼び出し側は `match` 等で**必ず成功/失敗の両方を処理**する（`#[must_use]` による警告付き）。
- 回復不能なエラーは `panic!` でプロセス終了（`unwrap` は内部で `panic`）。
- `?` 演算子（糖衣）: 関数内で `let x = foo()?;` と書くと「`Err` なら即 return」。書籍で頻出（今回は紹介のみ）。

> **C++では** 例外 / エラーコード / `std::expected`（C++23）が混在。  
> **Rustでは** 戻り値が必ず `Result<T, E>` で表現され、**制御フローに乗る**。

### サンプル

```rust
fn main() {
    // parse は Result を返す
    let r: Result<i32, _> = "42".parse();
    match r {
        Ok(n)  => println!("got {}", n),      // => got 42
        Err(e) => println!("parse error: {}", e),
    }

    // 失敗する例
    let r: Result<i32, _> = "abc".parse();
    match r {
        Ok(n)  => println!("got {}", n),
        Err(e) => println!("parse error: {}", e),
                                              // => parse error: invalid digit found in string
    }

    // ? 演算子（参考）: 関数の戻り値が Result のときに使える
    fn parse_add(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
        let x: i32 = a.parse()?;   // ← Err ならここで return
        let y: i32 = b.parse()?;
        Ok(x + y)
    }
    println!("{:?}", parse_add("1", "2"));    // => Ok(3)
    println!("{:?}", parse_add("1", "x"));    // => Err(ParseIntError { kind: InvalidDigit })
}
```

---

## 9. `enum` とパターンマッチング

### ポイント

- Rustの `enum` は**バリアントごとに異なるデータを持てる**（代数的データ型 / Sum Type）。
- 先に出た `Option` も `Result` も、実は標準ライブラリの `enum` の具体例。
- `match` は**網羅的**でなければならない。全バリアントを処理するかワイルドカード `_` が必須。
- パターンで値の分解（デストラクチャリング）もできる。
- `if let` / `while let` は「1パターンだけ興味がある」ときの糖衣。

> **C++では** `enum` は整数の別名。多態データには `std::variant` + `std::visit` だが扱いが煩雑。  
> **Rustでは** `enum` + `match` は言語の中心機能。`std::variant` に近いが**ずっと書きやすい**。

### サンプル

```rust
enum Shape {
    Circle(f64),        // 半径
    Rect(f64, f64),     // 幅・高さ
    Point,              // データなし
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r)  => std::f64::consts::PI * r * r,
        Shape::Rect(w, h) => w * h,
        Shape::Point      => 0.0,
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(1.0),
        Shape::Rect(2.0, 3.0),
        Shape::Point,
    ];
    for s in &shapes {
        println!("{}", area(s));
    }
    // => 3.141592653589793
    // => 6
    // => 0
}
```

---

## 10. `struct` と `impl` と `trait`（継承はない）

### ポイント

- データ構造は `struct`、振る舞い（メソッド）は `impl` ブロックで定義。
- **継承は存在しない**。共通の振る舞いは `trait`（インターフェース相当）で表現し、`impl Trait for Type` で実装する。
- `#[derive(Debug, Clone, PartialEq, ...)]` でよくある実装を自動生成できる。
- `Self::new(...)` は関連関数（≈ 静的メンバ関数）。`&self` / `&mut self` を取るものがメソッド。
- 設計は **is-a より has-a**、**継承よりトレイト境界**。

> **C++では** `class` + 仮想関数 + 多重継承（複雑化しがち）。  
> **Rustでは** `struct` + `trait` + ジェネリクス。契約ベースでシンプル。

### サンプル

```rust
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // 関連関数（コンストラクタ相当）
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // メソッド（&self = 不変借用）
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

trait Named {
    fn name(&self) -> &str;
}

impl Named for Point {
    fn name(&self) -> &str { "Point" }
}

fn main() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    println!("{}", a.distance(&b));   // => 5
    println!("{}", a.name());         // => Point
    println!("{:?}", a);              // => Point { x: 0.0, y: 0.0 }  ← Debug derive のおかげ
}
```

---

## 11. マクロと書籍頻出キーワード

### ポイント

- 末尾に `!` が付くもの（`println!`, `format!`, `vec!`, `assert!`, `assert_eq!`, `dbg!` など）は**マクロ**。
- C++のプリプロセッサマクロとは異なり、Rustのマクロは**構文木レベルで展開**される（型安全・スコープ安全）。
- 書式化はプレースホルダ方式。
  - `{}` = `Display` トレイト（人向け）
  - `{:?}` = `Debug` トレイト（開発者向け）
  - `{:#?}` = 整形表示（pretty-print）
- `use クレート名::{型, 関数}` で名前を取り込む。C++の `using` に近いが、`#include` とは違い**インポート**（本体の取り込みは不要）。

> **C++では** `printf` / `std::format` + `#include` でヘッダを取り込む。  
> **Rustでは** `println!` マクロ + `use`。`include` 相当は存在しない。

### サンプル

```rust
use std::collections::HashMap;

fn main() {
    // 書式化
    println!("{} + {} = {}", 1, 2, 1 + 2);      // => 1 + 2 = 3

    // Debug 出力
    let v = vec![1, 2, 3];
    println!("{:?}", v);                         // => [1, 2, 3]
    println!("{:#?}", v);
    // =>
    // [
    //     1,
    //     2,
    //     3,
    // ]

    // format! は文字列を作るだけ（出力はしない）
    let s: String = format!("x={}", 42);
    println!("{}", s);                           // => x=42

    // dbg! は値と位置情報を stderr に出しつつ値を返す（デバッグ用）
    let y = dbg!(1 + 2);                         // stderr: [src/main.rs:XX] 1 + 2 = 3
    println!("{}", y);                           // => 3

    // assert 系
    assert_eq!(2 + 2, 4);

    // use で取り込んだ型を使う
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("key", 1);
    println!("{:?}", m.get("key"));              // => Some(1)
}
```

---

## 付録

### A. `cargo test` と `#[test]`（書籍で頻出）

```rust
// src/main.rs または src/lib.rs
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }
}
```

```bash
$ cargo test
running 2 tests
test tests::test_add      ... ok
test tests::test_add_zero ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### B. よく出会うエラーメッセージ早見表

| エラー要旨 | 原因 | 対処 |
|---|---|---|
| `cannot assign twice to immutable variable` | `let` 束縛に再代入した | `let mut` に変える |
| `borrow of moved value` | ムーブ後の変数を使った | `.clone()` する or 借用（`&`）で渡す |
| `cannot borrow ... as mutable because it is also borrowed as immutable` | 不変借用の有効中に可変借用した | 不変借用のスコープを閉じてから可変借用 |
| `cannot borrow ... as mutable more than once at a time` | 可変借用が同時に2つ以上 | 片方のスコープを閉じる |
| `expected ... , found ()` | 末尾にセミコロンを付けて式を文にしてしまった | 末尾のセミコロンを外す |
| `expected struct X, found &X` | 所有型と参照型の取り違え | `&` を付ける / 外す |
| `missing lifetime specifier` | 参照を返す関数でライフタイムが推論できない | いったん所有型（`String`, `Vec`）で返す |

### C. 書籍導入で特に押さえたいキーワード対応表

| 書籍/Rustの用語 | C++の近い概念 | 要注意ポイント |
|---|---|---|
| `let` / `let mut` | `const T x` / `T x` | デフォルトが不変（逆） |
| ムーブ | `std::move` | Rustでは**暗黙**。元変数は使用不可 |
| 借用 `&T` / `&mut T` | `const T&` / `T&` | 借用規則で静的検証 |
| `String` / `&str` | `std::string` / `std::string_view` | 型の使い分けが必須 |
| `Vec<T>` / `&[T]` | `std::vector<T>` / `std::span<T>` | 同上 |
| `Option<T>` | `std::optional<T>` / ポインタ | nullが存在しない |
| `Result<T, E>` | `std::expected<T, E>` / 例外 | 例外機構なし |
| `enum` | `std::variant` | `match` で網羅的に処理 |
| トレイト | インターフェース（純粋仮想クラス） | 継承なし、契約ベース |
| `#[derive(...)]` | 自動生成の慣用句 | `Debug`, `Clone`, `PartialEq` など |
| クレート | ライブラリ + パッケージ | `crates.io` で検索 |
| マクロ（`!`） | 構文糖 / テンプレート | プリプロセッサではない |

