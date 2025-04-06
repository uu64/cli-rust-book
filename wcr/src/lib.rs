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

fn format_field(num: usize, show: bool) -> String {
    if show {
        format!("{:>8}", num)
    } else {
        "".to_string()
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_num_lines = 0;
    let mut total_num_words = 0;
    let mut total_num_bytes = 0;
    let mut total_num_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) =>  {
                let file_info = count(open(filename).unwrap())?;
                let num_lines = format_field(file_info.num_lines, config.lines);
                let num_words = format_field(file_info.num_words, config.words);
                let num_bytes = format_field(file_info.num_bytes, config.bytes);
                let num_chars = format_field(file_info.num_chars, config.chars);
                let filename = if filename == "-" { "".to_string() } else { format!(" {filename}") };
                println!("{num_lines}{num_words}{num_bytes}{num_chars}{filename}");

                total_num_lines += file_info.num_lines;
                total_num_words += file_info.num_words;
                total_num_bytes += file_info.num_bytes;
                total_num_chars += file_info.num_chars;
            },
        }
    }

    if config.files.len() > 1 {
        let total_num_lines = format_field(total_num_lines, config.lines);
        let total_num_words = format_field(total_num_words, config.words);
        let total_num_bytes = format_field(total_num_bytes, config.bytes);
        let total_num_chars = format_field(total_num_chars, config.chars);
        println!("{total_num_lines}{total_num_words}{total_num_bytes}{total_num_chars} total");
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
