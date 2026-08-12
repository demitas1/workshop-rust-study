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
