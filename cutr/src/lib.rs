use clap::Args;
use clap::Parser;
use std::{error::Error, ops::Range};


type MyResult<T> = Result<T, Box<dyn Error>>;

type PositionList = Vec<Range<usize>>;


#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
struct Extract {
    #[arg(short, long)]
    fields: String,
    #[arg(short, long)]
    bytes: String,
    #[arg(short, long)]
    chars: String,
}

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,
    // #[arg(default_value = ';', help = "Input file(s)")]
    // delimiter: u8,
    #[command(flatten)]
    extract: Extract,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config:Config) -> MyResult<()> {
    println!("{:#?}", &config);
    Ok(())
}

