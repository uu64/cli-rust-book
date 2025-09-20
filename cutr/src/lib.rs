use clap::Args;
use clap::Parser;
use regex::bytes::Regex;
use std::str::FromStr;
use std::sync::LazyLock;
use std::{error::Error, ops::Range};


type MyResult<T> = Result<T, Box<dyn Error>>;

static RE_POS_RANGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?<start>[1-9]|[0-9]{2,})(-(?<end>[1-9]|[0-9]{2,}))?$").unwrap()
});

#[derive(Clone, Debug, PartialEq, Eq)]
struct PosRange(Vec<Range<usize>>);

impl FromStr for PosRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges: Vec<Range<usize>> = vec![];

        for s in s.split(',') {
            if !RE_POS_RANGE.is_match(s.as_bytes()) {
                return Err(format!("illegal list value: \"{}\"", s));
            }

            if let Some((a, b)) = s.split_once('-') {
                let start: usize = a.parse().map_err(|_| format!("invalid start: {}", a))?;

                let end: usize = b.parse().map_err(|_| format!("invalid end: {}", b))?;
                if end <= start {
                    return Err(format!("First number in range ({}) must be lower than second number ({})", start, end));
                }
                ranges.push(start-1..end);
            } else {
                let n: usize = s.parse().map_err(|_| format!("invalid number: {}", s))?;
                ranges.push(n-1..n);
            }
        }
        Ok(PosRange(ranges))
    }
}

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
struct Extract {
    #[arg(short, long, help="Selected bytes")]
    bytes: Vec<PosRange>,
    #[arg(short, long, help="Selected characters")]
    chars: Vec<PosRange>,
    #[arg(short, long, help="Selected fields")]
    fields: Vec<PosRange>,
}

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(value_name="FILE", default_value="-", help="Input file(s)")]
    files: Vec<String>,
    #[arg(short, long, default_value_t=b'\t', help="Input file(s)")]
    delimiter: u8,
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

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(PosRange::from_str("").is_err());

        // Zero is an error
        let res = PosRange::from_str("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);
        let res = PosRange::from_str("0-1");
        assert!(res.is_err());
        // assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0-1""#);

        // A leading "+" is an error
        let res = PosRange::from_str("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "+1""#,);
        let res = PosRange::from_str("+1-2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#,
        );
        let res = PosRange::from_str("1-+2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#,
        );

        // Any non-number is an error
        let res = PosRange::from_str("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);
        let res = PosRange::from_str("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);
        let res = PosRange::from_str("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "1-a""#,);
        let res = PosRange::from_str("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a-1""#,);

        // Wonky ranges
        let res = PosRange::from_str("-");
        assert!(res.is_err());
        let res = PosRange::from_str(",");
        assert!(res.is_err());
        let res = PosRange::from_str("1,");
        assert!(res.is_err());
        let res = PosRange::from_str("1-");
        assert!(res.is_err());
        let res = PosRange::from_str("1-1-1");
        assert!(res.is_err());
        let res = PosRange::from_str("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = PosRange::from_str("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "First number in range (1) must be lower than second number (1)"
        );
        let res = PosRange::from_str("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = PosRange::from_str("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, vec![0..1]);
        let res = PosRange::from_str("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, vec![0..1]);
        let res = PosRange::from_str("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, vec![0..1, 2..3]);
        let res = PosRange::from_str("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, vec![0..1, 2..3]);
        let res = PosRange::from_str("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, vec![0..3]);
        let res = PosRange::from_str("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, vec![0..3]);
        let res = PosRange::from_str("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, vec![0..1, 6..7, 2..5]);
        let res = PosRange::from_str("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, vec![14..15, 18..20]);
    }
}

