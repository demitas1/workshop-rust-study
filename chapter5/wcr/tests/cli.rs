use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "wcr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";
const SPIDERSJ: &str = "tests/inputs/spidersj.txt";

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn dies_chars_and_bytes() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["-m", "-c"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "The argument '--bytes' cannot be used with '--chars'",
        ));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_bytes() -> TestResult {
    run(&["--bytes", FOX], "tests/expected/fox.txt.c.out")
}

// --------------------------------------------------
#[test]
fn fox_chars() -> TestResult {
    run(&["--chars", FOX], "tests/expected/fox.txt.m.out")
}

// --------------------------------------------------
#[test]
fn fox_words() -> TestResult {
    run(&["--words", FOX], "tests/expected/fox.txt.w.out")
}

// --------------------------------------------------
#[test]
fn fox_lines() -> TestResult {
    run(&["--lines", FOX], "tests/expected/fox.txt.l.out")
}

// --------------------------------------------------
#[test]
fn fox_words_bytes() -> TestResult {
    run(&["-w", "-c", FOX], "tests/expected/fox.txt.wc.out")
}

// --------------------------------------------------
#[test]
fn fox_words_lines() -> TestResult {
    run(&["-w", "-l", FOX], "tests/expected/fox.txt.wl.out")
}

// --------------------------------------------------
#[test]
fn fox_bytes_lines() -> TestResult {
    run(&["-l", "-c", FOX], "tests/expected/fox.txt.cl.out")
}

// --------------------------------------------------
#[test]
fn atlamal() -> TestResult {
    run(&[ATLAMAL], "tests/expected/atlamal.txt.out")
}

// --------------------------------------------------
#[test]
fn atlamal_bytes() -> TestResult {
    run(&["-c", ATLAMAL], "tests/expected/atlamal.txt.c.out")
}

// --------------------------------------------------
#[test]
fn atlamal_words() -> TestResult {
    run(&["-w", ATLAMAL], "tests/expected/atlamal.txt.w.out")
}

// --------------------------------------------------
#[test]
fn atlamal_lines() -> TestResult {
    run(&["-l", ATLAMAL], "tests/expected/atlamal.txt.l.out")
}

// --------------------------------------------------
#[test]
fn atlamal_words_bytes() -> TestResult {
    run(&["-w", "-c", ATLAMAL], "tests/expected/atlamal.txt.wc.out")
}

// --------------------------------------------------
#[test]
fn atlamal_words_lines() -> TestResult {
    run(&["-w", "-l", ATLAMAL], "tests/expected/atlamal.txt.wl.out")
}

// --------------------------------------------------
#[test]
fn atlamal_bytes_lines() -> TestResult {
    run(&["-l", "-c", ATLAMAL], "tests/expected/atlamal.txt.cl.out")
}

// --------------------------------------------------
#[test]
fn atlamal_stdin() -> TestResult {
    let input = fs::read_to_string(ATLAMAL)?;
    let expected =
        fs::read_to_string("tests/expected/atlamal.txt.stdin.out")?;
    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
// 空白を持たない日本語（小林一茶「隅の蜘蛛案じな煤はとらぬぞよ」）。
// 単語を「空白文字で区切られた文字列」と定義するため、単語数は 1 となる。
// 形態素単位の数え方はこのコマンドの責務としない。
// またマルチバイト文字のみで構成されるため、バイト数 43 と文字数 15 が食い違う。
#[test]
fn spidersj() -> TestResult {
    run(&[SPIDERSJ], "tests/expected/spidersj.txt.out")
}

// --------------------------------------------------
#[test]
fn spidersj_bytes() -> TestResult {
    run(&["-c", SPIDERSJ], "tests/expected/spidersj.txt.c.out")
}

// --------------------------------------------------
#[test]
fn spidersj_chars() -> TestResult {
    run(&["-m", SPIDERSJ], "tests/expected/spidersj.txt.m.out")
}

// --------------------------------------------------
#[test]
fn spidersj_words() -> TestResult {
    run(&["-w", SPIDERSJ], "tests/expected/spidersj.txt.w.out")
}

// --------------------------------------------------
#[test]
fn spidersj_lines() -> TestResult {
    run(&["-l", SPIDERSJ], "tests/expected/spidersj.txt.l.out")
}

// --------------------------------------------------
#[test]
fn spidersj_words_bytes() -> TestResult {
    run(&["-w", "-c", SPIDERSJ], "tests/expected/spidersj.txt.wc.out")
}

// --------------------------------------------------
#[test]
fn spidersj_words_chars() -> TestResult {
    run(&["-w", "-m", SPIDERSJ], "tests/expected/spidersj.txt.wm.out")
}

// --------------------------------------------------
#[test]
fn spidersj_words_lines() -> TestResult {
    run(&["-w", "-l", SPIDERSJ], "tests/expected/spidersj.txt.wl.out")
}

// --------------------------------------------------
#[test]
fn spidersj_bytes_lines() -> TestResult {
    run(&["-l", "-c", SPIDERSJ], "tests/expected/spidersj.txt.cl.out")
}

// --------------------------------------------------
#[test]
fn spidersj_stdin() -> TestResult {
    let input = fs::read_to_string(SPIDERSJ)?;
    let expected =
        fs::read_to_string("tests/expected/spidersj.txt.stdin.out")?;
    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all() -> TestResult {
    run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
}

// --------------------------------------------------
#[test]
fn test_all_lines() -> TestResult {
    run(&["-l", EMPTY, FOX, ATLAMAL], "tests/expected/all.l.out")
}

// --------------------------------------------------
#[test]
fn test_all_words() -> TestResult {
    run(&["-w", EMPTY, FOX, ATLAMAL], "tests/expected/all.w.out")
}

// --------------------------------------------------
#[test]
fn test_all_bytes() -> TestResult {
    run(&["-c", EMPTY, FOX, ATLAMAL], "tests/expected/all.c.out")
}

// --------------------------------------------------
#[test]
fn test_all_words_bytes() -> TestResult {
    run(&["-cw", EMPTY, FOX, ATLAMAL], "tests/expected/all.wc.out")
}

// --------------------------------------------------
#[test]
fn test_all_words_lines() -> TestResult {
    run(&["-wl", EMPTY, FOX, ATLAMAL], "tests/expected/all.wl.out")
}

// --------------------------------------------------
#[test]
fn test_all_bytes_lines() -> TestResult {
    run(&["-cl", EMPTY, FOX, ATLAMAL], "tests/expected/all.cl.out")
}
