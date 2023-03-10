mod owner;

use chrono::{DateTime, Local};
use clap::{App, Arg};
//use owner::Owner;
use std::{error::Error, fs, os::unix::fs::MetadataExt, path::PathBuf};
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .about("Rust ls")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Files and/or directories")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("long")
                .takes_value(false)
                .help("Long listing")
                .short("l")
                .long("long"),
        )
        .arg(
            Arg::with_name("all")
                .takes_value(false)
                .help("Show all files")
                .short("a")
                .long("all"),
        )
        .get_matches();

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        long: matches.is_present("long"),
        show_hidden: matches.is_present("all"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let paths = find_files(&config.paths, config.show_hidden)?;
    if config.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
    }

    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut result = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => eprintln!("{}: {}", path, e),
            Ok(metadata) => {
                if metadata.is_dir() {
                    let dir = fs::read_dir(path)?;
                    dir.into_iter().flatten().for_each(|f| {
                        let buf = f.path();

                        if show_hidden {
                            result.push(buf);
                        } else {
                            match buf.file_name() {
                                Some(file_name) => {
                                    if !file_name.to_string_lossy().starts_with(".") {
                                        result.push(buf)
                                    }
                                }
                                _ => (),
                            }
                        }
                    });
                } else {
                    result.push(PathBuf::from(path));
                }
            }
        };
    }

    Ok(result)
}

fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = fs::metadata(path)?;

        let cell_1 = if metadata.is_dir() { "d" } else { "-" };
        let cell_2 = format_mode(metadata.mode());
        let cell_3 = metadata.nlink();
        let (cell_4, cell_5) = get_user_and_group(&metadata);
        let cell_6 = metadata.len();
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
        let cell_7 = modified.format("%b %d %y %H:%M");
        let cell_8 = path.display();

        table.add_row(
            Row::new()
                .with_cell(cell_1)
                .with_cell(cell_2)
                .with_cell(cell_3)
                .with_cell(cell_4)
                .with_cell(cell_5)
                .with_cell(cell_6)
                .with_cell(cell_7)
                .with_cell(cell_8),
        );
    }

    Ok(format!("{}", table))
}

fn format_mode(mode: u32) -> String {
    let mut result = String::new();

    let mut push_mode = |mask: u32, mode_str: &str| {
        if mask != 0 {
            result.push_str(mode_str)
        } else {
            result.push_str("-")
        }
    };

    push_mode(mode & 0o400, "r");
    push_mode(mode & 0o200, "w");
    push_mode(mode & 0o100, "x");
    push_mode(mode & 0o040, "r");
    push_mode(mode & 0o020, "w");
    push_mode(mode & 0o010, "x");
    push_mode(mode & 0o004, "r");
    push_mode(mode & 0o002, "w");
    push_mode(mode & 0o001, "x");

    result
}

fn get_user_and_group(metadata: &fs::Metadata) -> (String, String) {
    let user = get_user_by_uid(metadata.uid())
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_default();
    let group = get_group_by_gid(metadata.gid())
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_default();

    (user, group)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_files() {
        // Find all non-hidden entries in a directory
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );

        // Any existing file should be found even if hidden
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // Test multiple path arguments
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        // Find all entries in a directory including hidden
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
    }

    fn long_match(
        line: &str,
        expected_name: &str,
        expected_perms: &str,
        expected_size: Option<&str>,
    ) {
        let parts: Vec<_> = line.split_whitespace().collect();
        assert!(parts.len() > 0 && parts.len() <= 10);

        let perms = parts.get(0).unwrap();
        assert_eq!(perms, &expected_perms);

        if let Some(size) = expected_size {
            let file_size = parts.get(4).unwrap();
            assert_eq!(file_size, &size);
        }

        let display_name = parts.last().unwrap();
        assert_eq!(display_name, &expected_name);
    }

    #[test]
    fn test_format_output_one() {
        let bustle_path = "tests/inputs/bustle.txt";
        let bustle = PathBuf::from(bustle_path);

        let res = format_output(&[bustle]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let lines: Vec<&str> = out.split("\n").filter(|s| !s.is_empty()).collect();
        assert_eq!(lines.len(), 1);

        let line1 = lines.first().unwrap();
        long_match(&line1, bustle_path, "-rw-r--r--", Some("193"));
    }

    #[test]
    fn test_format_output_two() {
        let res = format_output(&[
            PathBuf::from("tests/inputs/dir"),
            PathBuf::from("tests/inputs/empty.txt"),
        ]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let mut lines: Vec<&str> = out.split("\n").filter(|s| !s.is_empty()).collect();
        lines.sort();
        assert_eq!(lines.len(), 2);

        let empty_line = lines.remove(0);
        long_match(
            &empty_line,
            "tests/inputs/empty.txt",
            "-rw-r--r--",
            Some("0"),
        );

        let dir_line = lines.remove(0);
        long_match(&dir_line, "tests/inputs/dir", "drwxr-xr-x", None);
    }

    //        #[test]
    //        fn test_mk_triple() {
    //            assert_eq!(mk_triple(0o751, Owner::User), "rwx");
    //            assert_eq!(mk_triple(0o751, Owner::Group), "r-x");
    //            assert_eq!(mk_triple(0o751, Owner::Other), "--x");
    //            assert_eq!(mk_triple(0o600, Owner::Other), "---");
    //        }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }
}
