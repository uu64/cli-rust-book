use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Read}};

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
    for (i, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => {
                if config.files.len() > 1 {
                    if i != 0 {
                        println!();
                    }
                    println!("==> {filename} <==") 
                }

                if let Some(num_bytes) = config.bytes {
                    let out = read_first_n_bytes(reader, num_bytes)?;
                    print!("{out}");
                } else  {
                    let out = read_first_n_lines(reader, config.lines)?;
                    print!("{out}");
                }
            },
        }
    }
    Ok(())
}

fn read_first_n_bytes(reader: Box<dyn BufRead>, num_bytes: usize) -> MyResult<String> {
    let mut handle = reader.take(num_bytes as u64);
    let mut buffer = vec![0; num_bytes];
    let bytes_read = handle.read(&mut buffer)?;

    return Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
}

fn read_first_n_lines(mut reader: Box<dyn BufRead>, num_lines: usize) -> MyResult<String> {
    let mut lines = Vec::new();
    let mut n = 0;

    while n < num_lines {
        let mut buf = String::new();
        _ = reader.read_line(&mut buf);
        lines.push(buf.clone());
        buf.clear();
        n += 1;
    }

    return Ok(lines.join(""));
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
