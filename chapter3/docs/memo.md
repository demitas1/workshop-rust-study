## cargo-expand
[https://qiita.com/toyboot4e/items/ef3a730b482181c018bc](https://qiita.com/toyboot4e/items/ef3a730b482181c018bc)

- インストール

```bash
% cargo install cargo-expand
...
    Finished `release` profile [optimized] target(s) in 35.96s
  Installing /Users/tsuch/.cargo/bin/cargo-expand
   Installed package `cargo-expand v1.0.122` (executable `cargo-expand`)
```

- `#[derive(Debug)]` 展開結果:　Debugトレイトを自動的に実装

```rust
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}
#[automatically_derived]
impl ::core::fmt::Debug for Config {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Config",
            "files",
            &self.files,
            "number_lines",
            &self.number_lines,
            "number_nonblank_lines",
            &&self.number_nonblank_lines,
        )
    }
}
```

## Clap::Arg default value

[https://docs.rs/clap/latest/clap/struct.Arg.html#method.default_value](https://docs.rs/clap/latest/clap/struct.Arg.html#method.default_value)

## Clap::Arg group()
- `group()`: [https://docs.rs/clap/latest/clap/struct.Arg.html#method.group](https://docs.rs/clap/latest/clap/struct.Arg.html#method.group)
  - 同じグループに所属するArgは、同時に指定できなくなる (複数指定を許すこともできそう？)
- `ArgGroup`: [https://docs.rs/clap/latest/clap/struct.ArgGroup.html](https://docs.rs/clap/latest/clap/struct.ArgGroup.html)

```bash
% cargo run -- -n -b
...
error: The argument '--number-nonblank' cannot be used with one or more of the other specified arguments
```

## Clap::Arg conflicts_with
- [https://docs.rs/clap/latest/clap/struct.Arg.html#method.conflicts_with](https://docs.rs/clap/latest/clap/struct.Arg.html#method.conflicts_with)

## use std::io::{self, BufRead, BufReader}; の self?
- `std::io` を使用する、という意味
- `BufRead` や `BufReader` の場合は `BufRead::~` というふうに使えるが、 それ以外のモジュールも `io::~` というふうに使用することができるようになる
- 参考: https://users.rust-lang.org/t/what-does-self-mean-in-use/15559/2

## "?" について
https://google.github.io/comprehensive-rust/ja/error-handling/try-conversions.html

```rust
expression?
```

は、下記と同等:

```rust
match expression {
    Ok(value) => value,
    Err(err)  => return Err(From::from(err)),
}
```