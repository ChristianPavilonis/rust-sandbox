use ansi_term::Style;
use chrono::{Datelike, Local, NaiveDate, Weekday};
use clap::{App, Arg};
use itertools::izip;
use std::{error::Error, str::FromStr};

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

const LINE_WIDTH: usize = 22;
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("calr")
        .version("0.1.0")
        .about("Rust cal")
        .arg(
            Arg::with_name("month")
                .value_name("MONTH")
                .short("m")
                .help("Month name or number (1-12)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("show_current_year")
                .value_name("SHOW_YEAR")
                .short("y")
                .long("year")
                .help("Show whole current year")
                .conflicts_with_all(&["month", "year"])
                .takes_value(false),
        )
        .arg(
            Arg::with_name("year")
                .value_name("YEAR")
                .help("Year (1-9999)"),
        )
        .get_matches();

    let mut month = matches.value_of("month").map(parse_month).transpose()?;
    let mut year = matches.value_of("year").map(parse_year).transpose()?;
    let today = Local::today();

    if matches.is_present("show_current_year") {
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }

    Ok(Config {
        month,
        year: year.unwrap_or_else(|| today.year()),
        today: today.naive_local(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    if let Some(month) = config.month {
        println!(
            "{}",
            format_month(config.year, month, true, config.today).join("\n")
        )
    } else {
        println!("{:>32}", config.year);
        let months: Vec<_> = (1..=12)
            .into_iter()
            .map(|month| format_month(config.year, month, false, config.today))
            .collect();

        for (i, chunk) in months.chunks(3).enumerate() {
            if let [m1, m2, m3] = chunk {
                for lines in izip!(m1, m2, m3) {
                    println!("{}{}{}", lines.0, lines.1, lines.2);
                }
                if i < 3 {
                    println!();
                }
            }
        }
    }

    Ok(())
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    match val.parse() {
        Err(_) => Err(format!("Invalid integer \"{}\"", val).into()),
        Ok(parsed) => Ok(parsed),
    }
}

fn parse_year(year: &str) -> MyResult<i32> {
    let y: i32 = parse_int(year)?;
    let range = 1..=9999;

    if !range.contains(&y) {
        return Err(format!("year \"{}\" not in the range 1 through 9999", y).into());
    }

    Ok(y)
}

fn parse_month(month: &str) -> MyResult<u32> {
    let mut month_num: u32 = 0;

    for (i, mo) in MONTH_NAMES.iter().enumerate() {
        if mo.to_lowercase().starts_with(&month.to_lowercase()) {
            month_num = i as u32 + 1;
            break;
        }
    }

    if month_num == 0 {
        month_num = parse_int(month).map_err(|_| format!("Invalid month \"{}\"", month))?;
    }

    if !(1..=12).contains(&month_num) {
        return Err(format!("month \"{}\" not in the range 1 through 12", month_num).into());
    }

    Ok(month_num)
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let mut cal = vec![];

    let month_name = MONTH_NAMES[(month - 1) as usize];

    cal.push(format!(
            "{:^width$}  ",
        if print_year {
            format!("{month_name} {year}")
        } else {
            format!("{month_name}")
        },
        width = LINE_WIDTH - 2
    ));
    cal.push(format!("{}", "Su Mo Tu We Th Fr Sa  "));

    let mut iter = NaiveDate::from_ymd_opt(year, month, 1).unwrap().iter_days();

    let mut sun = String::new();
    let mut mon = String::new();
    let mut tue = String::new();
    let mut wed = String::new();
    let mut thu = String::new();
    let mut fri = String::new();
    let mut sat = String::new();
    // push dates.
    for _ in 0..6 {
        loop {
            let date = iter.next().unwrap();
            let day = if today == date {
                Style::new().reverse().paint(format!("{:>2}", date.day())).to_string()
            } else if date.month() == month {
                date.day().to_string()
            } else {
                String::new()
            };

            match date.weekday() {
                Weekday::Sun => sun = day,
                Weekday::Mon => mon = day,
                Weekday::Tue => tue = day,
                Weekday::Wed => wed = day,
                Weekday::Thu => thu = day,
                Weekday::Fri => fri = day,
                Weekday::Sat => {
                    sat = day;
                    cal.push(format!(
                        "{:>2} {:>2} {:>2} {:>2} {:>2} {:>2} {:>2}  ",
                        sun, mon, tue, wed, thu, fri, sat
                    ));
                    break;
                }
            };
        }
    }

    cal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        // Parse positive int as usize
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        // Parse negative int as i32
        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        // Fail on a string
        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_year() {
        let res = parse_year("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1i32);

        let res = parse_year("9999");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 9999i32);

        let res = parse_year("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"0\" not in the range 1 through 9999"
        );

        let res = parse_year("10000");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"10000\" not in the range 1 through 9999"
        );

        let res = parse_year("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1 through 12"
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1 through 12"
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd(0, 1, 1);
        let leap_february = vec![
            "   February 2020      ",
            "Su Mo Tu We Th Fr Sa  ",
            "                   1  ",
            " 2  3  4  5  6  7  8  ",
            " 9 10 11 12 13 14 15  ",
            "16 17 18 19 20 21 22  ",
            "23 24 25 26 27 28 29  ",
            "                      ",
        ];
        assert_eq!(format_month(2020, 2, true, today), leap_february);

        let may = vec![
            "        May           ",
            "Su Mo Tu We Th Fr Sa  ",
            "                1  2  ",
            " 3  4  5  6  7  8  9  ",
            "10 11 12 13 14 15 16  ",
            "17 18 19 20 21 22 23  ",
            "24 25 26 27 28 29 30  ",
            "31                    ",
        ];
        assert_eq!(format_month(2020, 5, false, today), may);

        let april_hl = vec![
            "     April 2021       ",
            "Su Mo Tu We Th Fr Sa  ",
            "             1  2  3  ",
            " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29 30     ",
            "                      ",
        ];

        println!("{}", april_hl.join("\n"));

        let today = NaiveDate::from_ymd(2021, 4, 7);
        assert_eq!(format_month(2021, 4, true, today), april_hl);
    }

    //        #[test]
    //        fn test_last_day_in_month() {
    //            assert_eq!(
    //                    last_day_in_month(2020, 1),
    //                NaiveDate::from_ymd(2020, 1, 31)
    //            );
    //            assert_eq!(
    //                    last_day_in_month(2020, 2),
    //                NaiveDate::from_ymd(2020, 2, 29)
    //            );
    //            assert_eq!(
    //                    last_day_in_month(2020, 4),
    //                NaiveDate::from_ymd(2020, 4, 30)
    //            );
    //        }
}
