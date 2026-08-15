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
