use chrono::{Datelike, Local, Month, NaiveDate};
use clap::Parser;
use shared_utils::MyResult;

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

    let month: Result<Month, _> = input.parse();
    if let Err(_) = month {
        return Err(format!("Invalid month \"{input}\"").into());
    }
    Ok(month.unwrap().number_from_month())
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

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    println!("{args:?}");
    Ok(())
}
