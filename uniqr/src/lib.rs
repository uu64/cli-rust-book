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
    let file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;


    let mut prev_line: Option<String> = None;
    let mut line_count = 0;

    let mut writer: BufWriter<Box<dyn Write>> = match config.out_file {
        Some(ref out_file) => BufWriter::new(Box::new(fs::File::create(out_file)?)),
        None => BufWriter::new(Box::new(io::stdout())),
    };

    let mut display = |line: &str, cnt: i32, conf: &Config| -> std::io::Result<()> {
        if conf.count {
            writer.write_all(format!("{:>4} ", cnt).as_bytes())?;
        }

        writer.write_all(format!("{line}\n").as_bytes())?;
        Ok(())
    };

    for line in file.lines() {
        let current = line?;

        if let Some(ref prev) = prev_line {
            if *prev != current {
                display(prev, line_count, &config)?;
                line_count = 0;
            }
        }

        prev_line = Some(current);
        line_count += 1;
    }

    if let Some(ref prev) = prev_line {
        display(prev, line_count, &config)?;
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
