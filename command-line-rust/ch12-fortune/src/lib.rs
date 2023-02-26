use clap::{App, Arg};
use rand::prelude::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};
use regex::{Regex, RegexBuilder};
use std::{
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

#[derive(Debug)]
pub struct Fortune {
    source: String,
    text: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("fortuner")
        .version("0.1.0")
        .about("Rust fortune")
        .arg(
            Arg::with_name("sources")
                .value_name("FILE")
                .multiple(true)
                .required(true)
                .help("Input files or directories"),
        )
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .short("m")
                .long("pattern")
                .help("Pattern"),
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .long("insensitive")
                .help("Case-insensitive pattern matching")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("seed")
                .value_name("SEED")
                .short("s")
                .long("seed")
                .help("Random seed"),
        )
        .get_matches();

    let sources = matches.values_of_lossy("sources").unwrap();
    let pattern = matches
        .value_of("pattern")
        .map(|val| {
            RegexBuilder::new(val)
                .case_insensitive(matches.is_present("insensitive"))
                .build()
                .map_err(|_| format!("Invalid --pattern \"{val}\""))
        })
        .transpose()?;
    let seed = matches.value_of("seed").map(parse_u64).transpose()?;

    Ok(Config {
        sources,
        pattern,
        seed,
    })
}

fn parse_u64(val: &str) -> MyResult<u64> {
    val.parse()
        .map_err(|_| format!("\"{val}\" not a valid integer").into())
}

pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.sources)?;
    let fortunes = read_fortunes(&files)?;
    let mut fortunes_found = false;



    if let Some(pattern) = config.pattern {
        let mut prev_source = String::new();

        for fortune in fortunes {
            if pattern.is_match(&fortune.text) {
                if fortune.source != prev_source {
                    prev_source = fortune.source;
                    eprintln!("({prev_source})\n%");
                }

                println!("{}", fortune.text);
                println!("%");
            }
        }
    } else {
        if let Some(f) = pick_fortune(&fortunes, config.seed) {
            println!("{f}");
        } else {
            println!("No fortunes found");
        }
    }

    Ok(())
}

fn find_files(paths: &[String]) -> MyResult<Vec<PathBuf>> {
    let mut path_bufs = vec![];

    for p in paths {
        match fs::metadata(p) {
            Err(e) => return Err(format!("{p}: {e}").into()),
            Ok(_) => {
                path_bufs.extend(
                    WalkDir::new(p)
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter(|f| f.file_type().is_file())
                        .map(|f| f.path().into()),
                );
            }
        };
    }

    path_bufs.sort();
    path_bufs.dedup();

    Ok(path_bufs)
}

fn read_fortunes(paths: &[PathBuf]) -> MyResult<Vec<Fortune>> {
    let mut fortunes = vec![];

    for path in paths {
        match File::open(path) {
            Err(e) => return Err(format!("{}: {}", path.to_str().unwrap_or(""), e).into()),
            Ok(file) => {
                let mut reader = BufReader::new(file);

                loop {
                    let mut buf = vec![];
                    let bytes = reader.read_until(b'%', &mut buf)?;
                    if bytes == 0 {
                        break;
                    }

                    buf.pop(); // remove %

                    let text = String::from_utf8_lossy(&buf).to_string().trim().to_string();

                    if !text.is_empty() {
                        fortunes.push(Fortune {
                            source: path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string(),
                            text,
                        });
                    }

                    buf.clear();
                }
            }
        }
    }

    Ok(fortunes)
}

fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    let mut rng = match seed {
        None => StdRng::from_entropy(),
        Some(seed) => StdRng::seed_from_u64(seed),
    };

    fortunes
        .choose(&mut rng)
        .map(|fortune| fortune.text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u64() {
        let res = parse_u64("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "\"a\" not a valid integer");

        let res = parse_u64("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);

        let res = parse_u64("4");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );

        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        // Test for multiple sources, path must be unique and sorted
        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }

    #[test]
    fn test_read_fortunes() {
        // Parses all the fortunes without a filter
        let res = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 6);
            assert_eq!(
                fortunes.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
                        A. Collared greens."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\n\
                        A: A bad idea (bad-eye deer)."
            );
        }

        // Filters for matching text
        let res = read_fortunes(&[
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 11);
    }

    #[test]
    fn test_pick_fortune() {
        // Create a slice of fortunes
        let fortunes = &[
            Fortune {
                source: "fortunes".to_string(),
                text: "You cannot achieve the impossible without \
                              attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Assumption is the mother of all screw-ups.".to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];

        // Pick a fortune with a seed
        assert_eq!(
            pick_fortune(fortunes, Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
