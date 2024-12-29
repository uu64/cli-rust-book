use std::error::Error;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about="Rust head", long_about = None)]
pub struct Config {
    #[arg(default_value="-")]
    files: Vec<String>,
    #[arg(short='n', long, default_value="10", conflicts_with("bytes"))]
    lines: usize,
    #[arg(short='c', long)]
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}