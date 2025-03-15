use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(default_value="-")]
    files: Vec<String>,
    #[arg(short='l', long, default_value_t=false)]
    lines: bool,
    #[arg(short='w', long, default_value_t=false)]
    words: bool,
    #[arg(short='c', long, default_value_t=false)]
    bytes: bool,
    #[arg(short='m', long, default_value_t=false, conflicts_with("bytes"))]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut buf = String::new();
    while let Ok(size) = file.read_line(&mut buf) {
        if size == 0 {
            // reached EOF
            break;
        }

        num_lines += 1;
        num_words += buf.split_ascii_whitespace().count();
        num_bytes += size;
        num_chars += buf.bytes().len();

        buf.clear();
    }
    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })

}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}

pub fn get_args() -> MyResult<Config> {
    let mut config: Config = Config::parse();

    if [
        config.lines,
        config.words,
        config.bytes,
        config.chars,
    ].iter().all(|v| v == &false) {
        config.lines = true;
        config.words = true;
        config.bytes = true;
    }

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) =>  {
                println!("Opened: {}", filename);
                _ = count(open(filename).unwrap());
            },
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
