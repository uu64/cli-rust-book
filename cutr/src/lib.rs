use clap::Args;
use clap::Parser;
use regex::bytes::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::sync::LazyLock;
use std::{error::Error, ops::Range};


type MyResult<T> = Result<T, Box<dyn Error>>;

static RE_POS_RANGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?<start>[1-9]|[0-9]{2,})(-(?<end>[1-9]|[0-9]{2,}))?$").unwrap()
});

#[derive(Clone, Debug, PartialEq, Eq)]
struct Position(Range<usize>);

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !RE_POS_RANGE.is_match(s.as_bytes()) {
            return Err(format!("illegal list value: \"{}\"", s));
        }

        if let Some((a, b)) = s.split_once('-') {
            let start: usize = a.parse().map_err(|_| format!("invalid start: {}", a))?;

            let end: usize = b.parse().map_err(|_| format!("invalid end: {}", b))?;
            if end <= start {
                return Err(format!("First number in range ({}) must be lower than second number ({})", start, end));
            }
            Ok(Position(start-1..end))
        } else {
            let n: usize = s.parse().map_err(|_| format!("invalid number: {}", s))?;
            Ok(Position(n-1..n))
        }
    }
}

fn parse_delimiter(s: &str) -> Result<u8, String> {
    let bytes = s.as_bytes();
    if bytes.len() != 1 {
        return Err(format!("--delim \"{}\" must be a single byte", s));
    }
    Ok(bytes[0])
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Extract {
    #[arg(short, long, value_delimiter=',', help="Selected fields")]
    fields: Vec<Position>,
    #[arg(short, long, value_delimiter=',', help="Selected bytes")]
    bytes: Vec<Position>,
    #[arg(short, long, value_delimiter=',', help="Selected characters")]
    chars: Vec<Position>,
}

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(value_name="FILE", default_value="-", help="Input file(s)")]
    files: Vec<String>,
    #[arg(value_name="DELIMITER", short, long, default_value="\t", help="Input file(s)", value_parser=parse_delimiter)]
    delim: u8,
    #[command(flatten)]
    extract: Extract,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config:Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => {
                println!("Opened {}", filename);
                extract_chars("hoge", config.extract.chars.as_slice());
            }
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

fn extract_chars(line: &str, char_ops: &[Position]) -> String {
    let mut s = String::from("");
    for range in char_ops {
        let start = match line.char_indices().nth(range.0.start) {
            Some(n) => n.0,
            None => line.len(),
        };
        let end = match line.char_indices().nth(range.0.end) {
            Some(n) => n.0,
            None => line.len(),
        };
        match line.get(start..end) {
            Some(sub) => s += sub,
            None => panic!("position is out of range"),
        }
    }
    s
}


#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(Position::from_str("").is_err());

        // Zero is an error
        let res = Position::from_str("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);
        let res = Position::from_str("0-1");
        assert!(res.is_err());
        // assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0-1""#);

        // A leading "+" is an error
        let res = Position::from_str("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "+1""#,);
        let res = Position::from_str("+1-2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#,
        );
        let res = Position::from_str("1-+2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#,
        );

        // Any non-number is an error
        let res = Position::from_str("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);
        // let res = PositionList::from_str("1,a");
        // assert!(res.is_err());
        // assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);
        let res = Position::from_str("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "1-a""#,);
        let res = Position::from_str("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a-1""#,);

        // Wonky ranges
        let res = Position::from_str("-");
        assert!(res.is_err());
        let res = Position::from_str(",");
        assert!(res.is_err());
        let res = Position::from_str("1,");
        assert!(res.is_err());
        let res = Position::from_str("1-");
        assert!(res.is_err());
        let res = Position::from_str("1-1-1");
        assert!(res.is_err());
        let res = Position::from_str("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = Position::from_str("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "First number in range (1) must be lower than second number (1)"
        );
        let res = Position::from_str("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = Position::from_str("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, 0..1);
        let res = Position::from_str("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, 0..1);
        // let res = PositionList::from_str("1,3");
        // assert!(res.is_ok());
        // assert_eq!(res.unwrap().0, vec![0..1, 2..3]);
        // let res = PositionList::from_str("001,0003");
        // assert!(res.is_ok());
        // assert_eq!(res.unwrap().0, vec![0..1, 2..3]);
        let res = Position::from_str("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, 0..3);
        let res = Position::from_str("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, 0..3);
        // let res = PositionList::from_str("1,7,3-5");
        // assert!(res.is_ok());
        // assert_eq!(res.unwrap().0, vec![0..1, 6..7, 2..5]);
        // let res = PositionList::from_str("15,19-20");
        // assert!(res.is_ok());
        // assert_eq!(res.unwrap().0, vec![14..15, 18..20]);
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[Position(0..1)]), "".to_string());
        assert_eq!(extract_chars("ábc", &[Position(0..1)]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[Position(0..1), Position(2..3)]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &[Position(0..3)]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[Position(2..3), Position(1..2)]), "cb".to_string());
        assert_eq!(extract_chars("ábc", &[Position(0..1), Position(1..2), Position(4..5)]), "áb".to_string());
    }
}

