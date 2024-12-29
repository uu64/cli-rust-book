use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
    .version("0.1.0")
    .author("uu64")
    .about("Rust cat")
    .arg(
        Arg::new("files")
        .value_name("FILE")
        .default_value("-")
        .help("Input file(s)")
        .num_args(1..)
    )
    .arg(
        Arg::new("number_lines")
        .long("number")
        .short('n')
        .action(ArgAction::SetTrue)
        .help("Number the output lines, starting at 1.")
    )
    .arg(
        Arg::new("number_nonblank_lines")
        .long("number-nonblank")
        .short('b')
        .action(ArgAction::SetTrue)
        .help("Number the non-blank output lines, starting at 1.")
        .conflicts_with("number_lines")
    )
    .get_matches();

    Ok(Config {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open{}: {}", filename, err),
            Ok(reader) => print(reader, config.number_lines, config.number_nonblank_lines),
        }
    }
    Ok(())
}

pub fn print(reader: Box<dyn BufRead>, number_lines: bool, number_nonblank_lines: bool) -> () {
    let mut line_number = 1;
    for line in reader.lines() {
        match line {
            Err(err) => panic!("failed to read line: {:?}", err),
            Ok(s) => {
                if number_nonblank_lines && s.is_empty() {
                    println!();
                    continue;
                }

                let mut prefix = String::from("");
                if number_lines || (number_nonblank_lines && !s.is_empty()) {
                    prefix.push_str(format!("{line_number:>6}\t").as_str());
                }
                println!("{prefix}{s}");
                line_number += 1;
            },
        }
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}