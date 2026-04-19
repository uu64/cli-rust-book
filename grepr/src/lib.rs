use std::{fs::File, io::{self, BufRead, BufReader}, result::Result::Ok};

use anyhow::{Result, anyhow};
use clap::Parser;
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(value_name = "PATTERN", help = "Search pattern")]
    pattern: String,
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, help = "Recursive search")]
    recursive: bool,
    #[arg(short, long, help = "Count occurrences")]
    count: bool,
    #[arg(short = 'v', long, help = "Invert match")]
    invert_match: bool,
    #[arg(short, long, help = "Case-insensitive")]
    insensitive: bool,
}

pub fn get_args() -> Result<Config> {
    let config = Config::parse();
    Ok(config)
}

pub fn run(config: Config) -> Result<()> {
    let pat = RegexBuilder::new(&config.pattern)
        .case_insensitive(config.insensitive)
        .build()
        .map_err(|e| anyhow!("Invalid pattern \"{}\": {}", config.pattern, e))?;
    for f in find_files(&config.files, config.recursive) {
        let f = match f {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut buf = open(&f).unwrap();
        let results = find_lines(&mut buf, &pat, config.invert_match);

        match results {
            Ok(results) => {
                let mut prefix = String::from("");
                if config.files.len() > 1 || config.recursive {
                    prefix = format!("{f}:");
                }
                if config.count {
                    println!("{}{}", prefix, results.len());
                } else {
                    for result in results {
                        print!("{prefix}{result}");
                    }
                }
            },
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    let mut results: Vec<Result<String>> = Vec::new();
    for path in paths {
        for entry in WalkDir::new(path) {
            let dir_entry = match entry {
                Ok(dir_entry) => dir_entry,
                Err(e) => {
                    results.push(Err(anyhow!(e)));
                    continue;
                }
            };

            let meta = match dir_entry.metadata() {
                Ok(meta) => meta,
                Err(e) => {
                    results.push(Err(anyhow!(e)));
                    continue;
                }
            };
            if meta.is_dir() {
                if !recursive {
                    results.push(Err(anyhow!(
                        "{} is a directory",
                        dir_entry.path().display()
                    )));
                    return results;
                }
                continue;
            }

            results.push(Ok(format!("{}", dir_entry.path().display())));
        }
    }
    results
}

fn find_lines<T: BufRead>(
    mut file: T,
    pattern: &Regex,
    invert_match: bool,
) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    let mut line = String::new();
    loop {
    // for line in file.lines() {
        let bytes = file.read_line(&mut line)?;
        // let butes = match bytes {
        //     Ok(b) => continue,
        //     Err(e) => return Err(anyhow!(e)),
        // };
        if bytes == 0 {
            break;
        }

        if pattern.is_match(&line) ^ invert_match {
            lines.push(line.clone()); // TODO: cloneいる？
        }
        line.clear();
    // }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::{find_files, find_lines};
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When inverted, the function should match the other two lines
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace('\\', "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}
