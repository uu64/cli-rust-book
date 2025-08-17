use clap::Parser;
use clap::ValueEnum;
use regex::Regex;
use std::error::Error;
use walkdir::DirEntry;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq, ValueEnum, Clone)]
enum EntryType {
    #[value(name = "f")]
    File,
    #[value(name = "d")]
    Dir,
    #[value(name = "l")]
    Link,
}

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(default_value = ".", help = "Search paths")]
    paths: Vec<String>,
    #[arg(
        value_name="NAME",
        short='n',
        long="name",
        help = "Name",
        num_args(0..),
    )]
    names: Vec<Regex>,
    #[arg(
        value_name="TYPE",
        short='t',
        long="type",
        help = "Entry type",
        num_args(0..),
        value_enum,
    )]
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();

    Ok(config)
}

fn is_match_entry_type(entry: &DirEntry, entry_types: &[EntryType]) -> bool {
    if entry_types.is_empty() {
        true
    } else if entry.file_type().is_dir() {
        entry_types.contains(&EntryType::Dir)
    } else if entry.file_type().is_file() {
        entry_types.contains(&EntryType::File)
    } else if entry.file_type().is_symlink() {
        entry_types.contains(&EntryType::Link)
    } else {
        false
    }
}

fn is_match_name(entry: &DirEntry, names: &[Regex]) -> bool {
    if names.is_empty() {
        return true;
    }
    match entry.file_name().to_str() {
        Some(fname) => {
            for regex in names {
                if regex.is_match(fname) {
                    return true;
                }
            }
            false
        }
        None => false,
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if is_match_entry_type(&entry, &config.entry_types) && is_match_name(&entry, &config.names) {
                        println!("{}", entry.path().display())
                    }
                }
            }
        }
    }
    Ok(())
}
