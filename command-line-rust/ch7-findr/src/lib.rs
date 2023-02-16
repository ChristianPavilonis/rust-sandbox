use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use walkdir::{WalkDir, DirEntry};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("your mom")
        .about("RUST FIND")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .possible_values(&["f", "d", "l"])
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let paths = matches.values_of_lossy("paths").unwrap();

    let names = matches
        .values_of_lossy("names")
        .map(|values| {
            values
                .into_iter()
                .map(|n| Regex::new(&n).map_err(|_| format!("Invalid --name \"{}\"", n)))
            .collect::<Result<Vec<_>, _>>()
        })
    .transpose()?
    .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .unwrap_or(Vec::new())
        .iter()
        .map(|t| match t.as_str() {
            "d" => Dir,
            "f" => File,
            "l" => Link,
            _ => unreachable!("Invalid type"),
        })
        .collect();

    Ok(Config {
        paths,
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{e}"),
                Ok(entry) => {
                    if is_requested_type(&config.entry_types, &entry) {

                        if config.names.is_empty() {
                            println!("{}", entry.path().display());
                        }
                        else {
                            config.names.iter().for_each(|name| {
                                if name.is_match(entry.file_name().to_str().expect("OH NO!")) {
                                    println!("{}", entry.path().display());
                                }
                            })
                        }
                    }
                },
            }
        }
    }

    Ok(())
}


fn is_requested_type(types: &Vec<EntryType>, entry: &DirEntry) -> bool {
    types.is_empty() || types.iter().any(|entry_type| match entry_type {
        Link => entry.file_type().is_symlink(),
        Dir => entry.file_type().is_dir(),
        File => entry.file_type().is_file(),
    })
}