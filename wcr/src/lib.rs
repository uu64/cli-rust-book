use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Read}};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(default_value="-")]
    files: Vec<String>,
    #[arg(short='l', long, default_value_t=false)]
    lines: bool,
    #[arg(short='w', long, default_value_t=false)]
    words: bool,
    #[arg(short='c', long, default_value_t=false)]
    bytes: bool,
    #[arg(short='m', long, default_value_t=false, conflicts_with("bytes"))]
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let mut config: Config = Config::parse();

    if [
        config.lines,
        config.words,
        config.bytes,
        config.chars,
    ].iter().all(|v| v == &false) {
        config.lines = true;
        config.words = true;
        config.bytes = true;
    }

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened: {}", filename),
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
