use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
                .help("Input Files"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("-number")
                .conflicts_with("number_nonblank_lines")
                .takes_value(false)
                .help("Number the output lines, starting at 1."),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("-number-nonblank")
                .takes_value(false)
                .help("Number the non-blank output lines, starting at 1."),
        )
        .before_help("Usage:")
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    Ok(Config {
        files: files,
        number_lines: number_lines,
        number_nonblank_lines: number_nonblank_lines,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf) => {
                let mut i = 1;
                for line in buf.lines() {
                    let line = line?;
                    if config.number_lines || (config.number_nonblank_lines && !line.is_empty()) {
                        print!("{:>6}\t", i);
                        i += 1;
                    }

                    println!("{}", line);
                }
            },
        }
    }
    Ok(())
}
