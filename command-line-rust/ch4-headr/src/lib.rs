use clap::{App, Arg};
use std::error::Error;

type MyReslut<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyReslut<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("your mom")
        .about("rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("input files")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("number of bytes")
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = match parse_positive_int(matches.value_of("lines").unwrap()) {
        Ok(n) => n,
        Err(invalid) => {
            return Err(format!("illegal line count -- {}", invalid).into());
        }
    };

    let bytes = match matches.value_of("bytes") {
        Some(value) => match parse_positive_int(value) {
            Ok(n) => Some(n),
            Err(invalid) => {
                return Err(format!("illegal byte count -- {}", invalid).into());
            }
        },
        None => None,
    };

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyReslut<()> {
    dbg!(config);

    Ok(())
}

fn parse_positive_int(val: &str) -> MyReslut<usize> {
    match val.parse() {
        Ok(number) if number > 0 => Ok(number),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0 is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
