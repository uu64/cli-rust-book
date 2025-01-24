use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

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
    let num_files = config.files.len();
    // let num_lines = config.lines;
    let num_bytes = config.bytes;

    for (i, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut reader) => {
                if num_files > 1 {
                    if i != 0 {
                        println!();
                    }
                    println!("==> {filename} <==") 
                }

                let mut buffer = Vec::new();
                _ = reader.read_to_end(&mut buffer)?;

                if num_bytes != None {
                    buffer.truncate(num_bytes.unwrap());
                }

                let s = String::from_utf8_lossy(&buffer);
                print!("{s}");
            },
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
