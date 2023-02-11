use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Kramer")
        .about("rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("out file"),
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .short("c")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    let in_file = matches.value_of_lossy("in_file").unwrap().to_string();
    let count = matches.is_present("count");

    let out_file = matches.value_of_lossy("out_file").map(String::from);

    Ok(Config {
        in_file,
        out_file,
        count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut line = String::new();
    let mut prev_line = String::new();
    let mut occurrences: usize = 1;
    let mut first_loop = true;
    let mut output = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            if ! prev_line.is_empty() {
                output = format!("{}{}", output, format(&config, occurrences, prev_line));
            }
            break;
        }

        if line.trim() == prev_line.trim() {
            occurrences += 1;
        } else {
            if first_loop {
                first_loop = false;
            } else {

                output = format!("{}{}", output, format(&config, occurrences, prev_line));
                occurrences = 1
            }

            prev_line = line.clone();
        }

        line.clear();
    }

    match config.out_file {
        Some(path) => {
            let mut file = File::create(&path)?;
            write!(file, "{}", output);
        }
        None => print!("{}", output),
    }
    Ok(())
}

fn format(config: &Config, occurrences: usize, line: String) -> String {
    if config.count {
        format!("{:>4} {}", occurrences, line)
    } else {
        format!("{}", line)
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
