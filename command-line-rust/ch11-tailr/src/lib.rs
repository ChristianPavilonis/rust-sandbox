use crate::TakeValue::*;
use clap::{App, Arg};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust tail")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Suppress headers"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = parse_num(matches.value_of("lines").unwrap())
        .map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = match matches.value_of("bytes") {
        None => None,
        Some(bytes) => Some(parse_num(bytes).map_err(|e| format!("illegal byte count -- {}", e))?),
    };
    let quiet = matches.is_present("quiet");

    Ok(Config {
        files,
        lines,
        bytes,
        quiet,
    })
}

pub fn run(config: Config) -> MyResult<()> {


    let print_headers = config.files.len() > 1 && !config.quiet;

    let mut count = 0;
    let total_files = config.files.len();

    for filename in config.files {
        match File::open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let (total_lines, total_bytes) = count_lines_bytes(&filename)?;
                let reader = BufReader::new(file);

                if print_headers {
                    println!("==> {} <==", filename);
                }

                match &config.bytes {
                    Some(bytes) => print_bytes(reader, bytes, total_bytes)?,
                    None => print_lines(reader, &config.lines, total_lines)?,
                }

                if print_headers && total_files - 1 > count {
                    println!();
                }

                count += 1;
            }
        }
    }

    Ok(())
}

fn parse_num(val: &str) -> MyResult<TakeValue> {
    if val == "+0" {
        return Ok(PlusZero);
    }

    let should_be_positive = val.starts_with("+");
    let parsed: i64 = val.parse().map_err(|_| val)?;

    if should_be_positive || parsed.is_negative() {
        Ok(TakeNum(parsed))
    } else {
        Ok(TakeNum(parsed * -1))
    }
}

fn count_lines_bytes(filename: &str) -> MyResult<(i64, i64)> {
    let lines = BufReader::new(File::open(filename)?).lines();
    let bytes = BufReader::new(File::open(filename)?).bytes();

    Ok((lines.count() as i64, bytes.count() as i64))
}

fn print_lines(mut file: impl BufRead, num_lines: &TakeValue, total_lines: i64) -> MyResult<()> {
    let start_index = get_start_index(num_lines, total_lines);

    match start_index {
        None => Ok(()),
        Some(start) => {
            let mut line = String::new();

            let mut count = 0;

            loop {
                let bytes = file.read_line(&mut line)?;

                if bytes == 0 {
                    break;
                }

                if count >= start {
                    print!("{line}");
                }

                count += 1;
                line.clear();
            }

            Ok(())
        }
    }
}

fn print_bytes<T>(mut file: T, numbytes: &TakeValue, total_bytes: i64) -> MyResult<()>
where
    T: Read + Seek,
{
    let start_index = get_start_index(numbytes, total_bytes);

    match start_index {
        None => Ok(()),
        Some(start) => {
            let bytes = file.bytes();

            let collected: Vec<_> = bytes.skip(start as usize).flatten().collect();

            print!("{}", String::from_utf8_lossy(&collected));

            Ok(())
        }
    }
}

fn get_start_index(take_val: &TakeValue, total: i64) -> Option<u64> {
    if total == 0 {
        return None;
    }

    match take_val {
        PlusZero => Some(0),
        TakeNum(num) => {
            if num == &0 {
                None
            } else {
                if num > &total {
                    None
                } else if num == &total {
                    Some((num - 1) as u64)
                } else if num < &0 && num.abs() < total {
                    Some((total + num) as u64)
                } else if num < &0 && num.abs() > total || num.abs() == total {
                    Some(0)
                } else {
                    Some((num.abs() - 1) as u64)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (1, 24));

        let res = count_lines_bytes("tests/inputs/ten.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (10, 49));
    }

    #[test]
    fn test_get_start_index() {
        // +0 from an empty file (0 lines/bytes) returns None
        assert_eq!(get_start_index(&PlusZero, 0), None);

        // +0 from a nonempty file returns an index that
        // is one less than the number of lines/bytes
        assert_eq!(get_start_index(&PlusZero, 1), Some(0));

        // Taking 0 lines/bytes returns None
        assert_eq!(get_start_index(&TakeNum(0), 1), None);

        // Taking any lines/bytes from an empty file returns None
        assert_eq!(get_start_index(&TakeNum(1), 0), None);

        // Taking more lines/bytes than is available returns None
        assert_eq!(get_start_index(&TakeNum(2), 1), None);

        // When starting line/byte is less than total lines/bytes,
        // return one less than starting number
        assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
        assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
        assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

        // When starting line/byte is negative and less than total,
        // return total - start
        assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
        assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
        assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

        // When the starting line/byte is negative and more than the total,
        // return 0 to print the whole file
        assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
    }

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = parse_num("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // A leading "+" should result in a positive number
        let res = parse_num("+3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        // An explicit "-" value should result in a negative number
        let res = parse_num("-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // Zero is zero
        let res = parse_num("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        // Plus zero is special
        let res = parse_num("+0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        // Test boundaries
        let res = parse_num(&i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&(i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(&i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        // A floating-point value is invalid
        let res = parse_num("3.14");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any non-integer string is invalid
        let res = parse_num("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}
