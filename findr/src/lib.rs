use clap::Parser;
use clap::ValueEnum;
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq, ValueEnum, Clone)]
enum EntryType {
    #[value(name="f")]
    File,
    #[value(name="d")]
    Dir,
    #[value(name="l")]
    Link,
}

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(default_value=".", help = "Search paths")]
    paths: Vec<String>,
    #[arg(value_name="NAME", short='n', long="name", help = "Name")]
    names: Vec<Regex>,
    #[arg(value_name="TYPE", value_enum, short='t', long="type", help = "Entry type")]
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
