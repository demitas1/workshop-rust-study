# memo

## 5.2 イテレータとクロージャ

```rust
if [lines, words, bytes, chars].iter().all(|v| v == &false) {
    lines = false;
    words = false;
    bytes = false;
}
```

`iter()` はスライスのメソッドで、`v` が参照となるため、比較対象を `&false` とする必要がある。

```console
62 |     if [lines, words, bytes, chars].iter().all(|v| v == false) {
   |                                                      ^^ no implementation for `&bool == bool`
```


値で取り出すために `into_iter()` を使用することができる。

```rust
if [lines, words, bytes, chars].into_iter().all(|v| v == false) {
    lines = false;
    words = false;
    bytes = false;
}
```

## 5.2.1 (復習) ? 演算子

```rust
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
```

`?` 「成功なら中身を取り出し、失敗なら即 return」

`?` を使わずに書くと：
```rust
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => {
            let file = match File::open(filename) {
                Ok(f) => f,                                 // 成功 → 中身を取り出す
                Err(e) => return Err(From::from(e)),        // 失敗 → その場でエラーをreturn
            };
            Ok(Box::new(BufReader::new(file)))
        }
    }
}
```

## 5.3.2 (復習) パターンマッチ

```rust
match count(file) {
    Ok(info) => {
        println!("{:?}", info);
    },
    _ => {}
}
```

**エラーに興味がない場合の省略記法 `if let`**

```rust
if let Ok(info) = count(file) {
    println!("{:?}", info);
}
```

**ココで `?` を使うとエラーの場合即座に終了し、次のファイルに実行が移らない**
```rust
println!("{:?}", count(file)?);
```

## 5.3.2 解答

- **dies_chars_and_bytes()**

```diff
@@ -61,7 +61,8 @@ pub fn get_args() -> MyResult<Config> {
             .short("m")
             .long("chars")
             .help("Show character count")
-            .takes_value(false),
+            .takes_value(false)
+            .conflicts_with("bytes")
         )
         .get_matches();
```

- **skips_bad_file()**

(`fn run()` で対処済み)

- **empty**

出力をフォーマットする

```diff
@@ -146,7 +147,15 @@ pub fn run(config: Config) -> MyResult<()> {
         match open(filename) {
             Err(err) => eprintln!("{}: {}", filename, err),
             Ok(file) => {
-                println!("{:?}", count(file)?);
+                if let Ok(info) = count(file) {
+                    println!(
+                        "{:>8}{:>8}{:>8} {}",
+                        info.num_lines,
+                        info.num_words,
+                        info.num_bytes,
+                        filename
+                    );
+                }
             }
         }
     }
```

[フォーマットについて (Rust By Example)](https://doc.rust-jp.rs/rust-by-example-ja/hello/print.html)

- **fox**

(注)

`fox` にマッチするテストが実行されるので、
    `fox_bytes`
    `fox_bytes_lines`
    `fox_chars`
    `fox_lines`
    `fox_words`
    `fox_words_bytes`
    `fox_words_lines`
すべてが実行される。

`fox` のみを実行するには
```bash
cargo test fox -- --exact
```

`format_field()` を定義して出力を `config` で制御

```diff
+fn format_field(value: usize, show: bool) -> String {
+    if show {
+        format!("{:>8}", value)
+    } else {
+        "".to_string()
+    }
+}
```

```diff
         match open(filename) {
             Err(err) => eprintln!("{}: {}", filename, err),
             Ok(file) => {
-                println!("{:?}", count(file)?);
+                if let Ok(info) = count(file) {
+                    println!(
+                        "{}{}{}{}{}",
+                        format_field(info.num_lines, config.lines),
+                        format_field(info.num_words, config.words),
+                        format_field(info.num_bytes, config.bytes),
+                        format_field(info.num_chars, config.chars),
+                        if filename == "-" {
+                            "".to_string()
+                        } else {
+                            format!(" {}", filename)
+                        }
+                    );
+                }
             }
         }
```

- **test_all**

合計を計算し、複数入力のときだけ出力する。

```diff
 pub fn run(config: Config) -> MyResult<()> {
+    let mut total_lines = 0;
+    let mut total_words = 0;
+    let mut total_bytes = 0;
+    let mut total_chars = 0;
+
     for filename in &config.files {
         match open(filename) {
             Err(err) => eprintln!("{}: {}", filename, err),
@@ -174,9 +179,24 @@ pub fn run(config: Config) -> MyResult<()> {
                             format!(" {}", filename)
                         }
                     );
+                    total_lines += info.num_lines;
+                    total_words += info.num_words;
+                    total_bytes += info.num_bytes;
+                    total_chars += info.num_chars;
                 }
             }
         }
     }
+
+    if config.files.len() > 1 {
+        println!(
+            "{}{}{}{} total",
+            format_field(total_lines, config.lines),
+            format_field(total_words, config.words),
+            format_field(total_bytes, config.bytes),
+            format_field(total_chars, config.chars),
+        );
+    }
+
     Ok(())
 }
```

## 5.4 iswspace(), GNU/BSD での動作の相違について

spidersj.txt のバイト列（43 バイト = 14 文字 × 3 + 改行、書籍の 43 と一致）:

```console
00000000  e9 9a 85  e3 81 ae  e8 9c 98  e8 9b 9b  e6 a1 88  e3
          └─ 隅 ─┘  └─ の ─┘  └─ 蜘 ─┘  └─ 蛛 ─┘  └─ 案 ─┘  └
00000010  81 98  e3 81 aa  e7 85 a4  e3 81 af  e3 81 a8  e3 82
          ─じ┘   └─ な ─┘  └─ 煤 ─┘  └─ は ─┘  └─ と ─┘  └ ら─
00000020  89  e3 81 ac  e3 81 9e  e3 82 88  0a
          ─┘  └─ ぬ ─┘  └─ ぞ ─┘  └─ よ ─┘
```

この中に `0x85` が 2 個。

- 隅 = `E9 9A 85` の 3 バイト目
- 煤 = `E7 85 A4` の 2 バイト目

**0x85 の正体**

`U+0085` は `NEL（NEXT LINE）` という C1 制御文字で、Unicode の `White_Space` プロパティが Yes に設定。

「Unicode 準拠の空白判定テーブル」を持つ `iswspace()` は、この値に対して真を返す。

BSD版 `wc` ではUTF-8デコードをせず、バイト単位で文字を検査している？

とりあえず、日本語のように分かち書きをしない言語で「単語」を正しく判定するのは `wc` コマンドの責任範囲外として、`spidersj.txt` に対してGNU版の動作を正解とするテストを書く。

```diff
--- /dev/null
+++ b/chapter5/wcr/tests/expected/spidersj.txt.stdin.out
@@ -0,0 +1 @@
+       1       1      43
```

```diff
--- /dev/null
+++ b/chapter5/wcr/tests/inputs/spidersj.txt
@@ -0,0 +1 @@
+隅の蜘蛛案じな煤はとらぬぞよ
```

```diff
+const SPIDERSJ: &str = "tests/inputs/spidersj.txt";

 // --------------------------------------------------
 fn gen_bad_file() -> String {
@@ -171,6 +172,77 @@ fn atlamal_stdin() -> TestResult {
     Ok(())
 }

+// --------------------------------------------------
+// 空白を持たない日本語（小林一茶「隅の蜘蛛案じな煤はとらぬぞよ」）。
+// 単語を「空白文字で区切られた文字列」と定義するため、単語数は 1 となる。
+// 形態素単位の数え方はこのコマンドの責務としない。
+// マルチバイト文字のみで構成されるため、バイト数 43 と文字数 15 が食い違う。
+#[test]
+fn spidersj() -> TestResult {
+    run(&[SPIDERSJ], "tests/expected/spidersj.txt.out")
+}

//(他のオプションについては同様なので省略)
```
