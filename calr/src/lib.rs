use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use clap::Parser;
use shared_utils::MyResult;

const ROW_WIDTH: usize = 20;
const WEEK: &'static str = "Su Mo Tu We Th Fr Sa ";
const YEAR_HEADER_WIDTH: usize = 32; // Three months of ROW_WIDTH with a space separator

const MONTHS: [&str; 12] = [
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

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct RawArgs {
    /// Month name or number
    #[arg(short, long, conflicts_with = "whole_year")]
    month: Option<String>,

    /// Year (1-9999)
    #[arg(conflicts_with = "whole_year")]
    year: Option<i32>,

    /// Show whole current year
    #[arg(
        short = 'y',
        long = "year",
        conflicts_with = "year",
        conflicts_with = "month"
    )]
    whole_year: bool,
}

#[derive(Debug)]
struct Args {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

fn parse_month(input: &str) -> MyResult<u32> {
    let month: Result<u32, _> = input.parse();
    if let Ok(number) = month {
        if (1u32..=12u32).contains(&number) {
            return Ok(number);
        } else {
            return Err(format!("month \"{number}\" not in the range 1 through 12").into());
        }
    }

    let month: Vec<_> = MONTHS.iter().enumerate().filter(|(_, s)| s.to_lowercase().starts_with(input)).collect();
    if month.len() != 1 {
        return Err(format!("Invalid month \"{input}\"").into());
    }
    Ok((month[0].0 + 1) as u32)
}

fn get_args() -> MyResult<Args> {
    let raw = RawArgs::parse();
    let today = Local::now().date_naive();

    let year = if let Some(input) = raw.year {
        if input < 1 || input > 9999 {
            return Err(From::from(format!(
                "year \"{input}\" not in the range 1 through 9999"
            )));
        }
        input
    } else {
        today.year()
    };
    let month = if let Some(input) = raw.month {
        Some(parse_month(&input)?)
    } else {
        if raw.whole_year || raw.year.is_some() {
            None
        } else {
            Some(today.month())
        }
    };
    Ok(Args {
        month: month,
        year: year,
        today: today,
    })
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> [String; 8] {
    let mut result: [String; 8] = Default::default(); // 2 headers and up to 6 weeks

    let month_name = MONTHS[(month - 1) as usize];
    let header = if print_year {
        format!(
            "{:^width$} ",
            format!("{} {}", month_name, year),
            width = ROW_WIDTH
        )
    } else {
        format!("{:^width$} ", month_name, width = ROW_WIDTH)
    };
    result[0] = header;


    result[1] = String::from(WEEK);

    let day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let week = day.week(Weekday::Sun);
    for w in 0usize..6 {
        let week = (week.first_day() + Duration::weeks(w as i64)).week(Weekday::Sun);
        let week_row: String = (0..7)
            .into_iter()
            .map(|i| {
                let day = week.first_day() + Duration::days(i);
                if day.month() == month {
                    if day == today {
                        format!("\u{1b}[7m{:>2}\u{1b}[0m ", day.day())
                    } else {
                        format!("{:>2} ", day.day())
                    }
                } else {
                    String::from("   ")
                }
            })
            .collect();
        result[w + 2] = week_row;
    }
    return result;
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    if let Some(month) = args.month {
        let formatted = format_month(args.year, month, true, args.today);
        for s in formatted {
            println!("{s}");
        }
        return Ok(());
    }

    println!("{:>width$}", args.year, width = YEAR_HEADER_WIDTH);

    let months: Vec<u32> = (1u32..=12u32).collect();

    for months_row in months.chunks_exact(3) {
        let formatted_months: Vec<[String; 8]> = months_row
            .into_iter()
            .map(|m| format_month(args.year, *m, false, args.today))
            .collect();
        for i in 0..8 {
            println!("{} {} {}", formatted_months[0][i], formatted_months[1][i], formatted_months[2][i])
        }
        println!();
    }

    Ok(())
}

#[test]
fn test_format_month() {
    let today = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
    let leap_february = [
        "   February 2020     ",
        "Su Mo Tu We Th Fr Sa ",
        "                   1 ",
        " 2  3  4  5  6  7  8 ",
        " 9 10 11 12 13 14 15 ",
        "16 17 18 19 20 21 22 ",
        "23 24 25 26 27 28 29 ",
        "                     ",
    ];
    assert_eq!(format_month(2020, 2, true, today), leap_february);

    let may = [
        "        May          ",
        "Su Mo Tu We Th Fr Sa ",
        "                1  2 ",
        " 3  4  5  6  7  8  9 ",
        "10 11 12 13 14 15 16 ",
        "17 18 19 20 21 22 23 ",
        "24 25 26 27 28 29 30 ",
        "31                   ",
    ];
    assert_eq!(format_month(2020, 5, false, today), may);

    let april_hl = [
        "     April 2021      ",
        "Su Mo Tu We Th Fr Sa ",
        "             1  2  3 ",
        " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10 ",
        "11 12 13 14 15 16 17 ",
        "18 19 20 21 22 23 24 ",
        "25 26 27 28 29 30    ",
        "                     ",
    ];
    let today = NaiveDate::from_ymd_opt(2021, 4, 7).unwrap();
    assert_eq!(format_month(2021, 4, true, today), april_hl);
}
