use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(default_value = "-", help = "Input file")]
    in_file: String,
    #[arg(help = "Output file")]
    out_file: Option<String>,
    #[arg(short = 'c', long, default_value_t = false, help = "Show counts")]
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();

    let mut prev_line: Option<String> = None;
    // let mut cnt = 0;
    loop {
        let bytes = file.read_line(&mut line)?;

        match prev_line {
            Some(prev) if bytes == 0 || prev != line => {
                print!("{}", prev);
            },
            _ => (), // do nothing
        }

        if bytes == 0 {
            break;
        }

        prev_line = Some(line.clone());
        line.clear();
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
