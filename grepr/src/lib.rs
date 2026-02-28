use std::error::Error;

use clap::Parser;
use regex::{Regex, RegexBuilder};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "PATTERN", help = "Search pattern")]
    pattern: String,
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, help = "Recursive search")]
    recursive: bool,
    #[arg(short, long, help = "Count occurrences")]
    count: bool,
    #[arg(short = 'v', long, help = "Invert match")]
    invert_match: bool,
    #[arg(short, long, help = "Case-insensitive")]
    insensitive: bool,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    let _ = RegexBuilder::new(&config.pattern)
        .case_insensitive(config.insensitive)
        .build()
        .map_err(|e| format!("Invalid pattern \"{}\": {}", config.pattern, e))?;
    Ok(())
}
