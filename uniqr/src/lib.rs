use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(default_value = "-", help = "Input file")]
    in_file: String,
    #[arg(help = "Output file")]
    out_file: Option<String>,
    #[arg(short = 'c', long, default_value_t = false, help = "Show counts")]
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    let mut writer: BufWriter<Box<dyn Write>> = match config.out_file {
        Some(ref out_file) => BufWriter::new(Box::new(fs::File::create(out_file)?)),
        None => BufWriter::new(Box::new(io::stdout())),
    };

    let mut write = |line: &str, cnt: i32, conf: &Config| -> std::io::Result<()> {
        if conf.count {
            writer.write_all(format!("{:>4} ", cnt).as_bytes())?;
        }
        writer.write_all(line.as_bytes())?;
        Ok(())
    };

    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut line_count = 0;
    let mut current: Option<String> = None;

    loop {
        let bytes = file.read_line(&mut line)?;

        if let Some(ref cur) = current {
            let newline_delimiters = ['\r', '\n'];
            if cur.trim_end_matches(newline_delimiters) != line.trim_end_matches(newline_delimiters)
                || bytes == 0
            {
                write(cur, line_count, &config)?;
                line_count = 0;
                current = Some(line.clone());
            }
        } else {
            // initialize
            current = Some(line.clone());
        }

        if bytes == 0 {
            break;
        }

        line_count += 1;
        line.clear();
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
