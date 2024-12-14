use std::error::Error;

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
        Arg::new("file")
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
    )
    .get_matches();

    let number_lines = matches.get_flag("number_lines");
    let number_nonblank_lines = matches.get_flag("number_nonblank_lines");

    if number_lines && number_nonblank_lines {
        Err("error: The argument '--number-nonblank' cannot be used with '--number'")?
    }

    Ok(Config {
        files: matches.get_many("file").unwrap().cloned().collect(),
        number_lines: number_lines,
        number_nonblank_lines: number_nonblank_lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}