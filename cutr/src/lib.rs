use std::{io::BufRead, num::NonZeroUsize, ops::Range};

use clap::Parser;
use regex::Regex;
use shared_utils::MyResult;

#[derive(Parser, Debug, Clone)]
#[group(required = true, multiple = false)]
struct RawExtract {
    /// Select only these bytes
    #[arg(short, long, value_delimiter = ',')]
    bytes: Vec<String>,

    /// Select only these characters
    #[arg(short, long, value_delimiter = ',')]
    characters: Vec<String>,

    /// Select only these fields
    #[arg(short, long, value_delimiter = ',')]
    fields: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(author, about, version)]
struct RawArgs {
    /// Files to cut from
    files: Vec<String>,

    #[command(flatten)]
    extract: RawExtract,

    /// Use DELIM instead of TAB for field delimeter
    #[arg(short, long, default_value_t = '\t')]
    delim: char,
}

#[derive(Debug)]
enum Extract {
    Bytes(Vec<Range<usize>>),
    Chars(Vec<Range<usize>>),
    Fields(Vec<Range<usize>>, char),
}

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    extract: Extract,
}

fn parse_range(value: &str) -> MyResult<Range<usize>> {
    let split: Vec<&str> = value.split('-').collect();
    match split.len() {
        1 => {
            let start = split[0].parse()?;
            Ok(start..start + 1)
        }
        2 => {
            let start: usize = split[0].parse()?;
            let end: usize = split[1].parse::<usize>()? + 1;
            Ok(start..end)
        }
        _ => Err(Box::from("not enough values")),
    }
}

fn parse_ranges(values: &[String]) -> MyResult<Vec<Range<usize>>> {
    values
        .iter()
        .map(|v| parse_range(v).map_err(|e| format!("illegal list value: \"{v}\", error: '{e}'")))
        .collect::<Result<Vec<Range<usize>>, _>>()
        .map_err(From::from)
}

fn get_args() -> MyResult<Args> {
    let args = RawArgs::parse();
    let extract = if !args.extract.bytes.is_empty() {
        Extract::Bytes(parse_ranges(&args.extract.bytes)?)
    } else if !args.extract.characters.is_empty() {
        Extract::Chars(parse_ranges(&args.extract.characters)?)
    } else if !args.extract.fields.is_empty() {
        Extract::Fields(parse_ranges(&args.extract.fields)?, args.delim)
    } else {
        return Err(Box::from("Failed to extract ranges"));
    };
    Ok(Args {
        extract,
        files: args.files,
    })
}

fn process_ranges<T>(
    elements: &mut dyn Iterator<Item = T>,
    ranges: &[Range<usize>],
    stringify: impl Fn(Vec<T>) -> String,
) -> Vec<String> {
    let mut counter = 1;
    let mut values: Vec<String> = Vec::new();
    for range in ranges {
        if counter >= range.end {
            continue;
        }
        if counter < range.start {
            elements.take(range.start - counter).for_each(|_| ());
            counter = range.start;
        }
        let val: Vec<_> = elements.take(range.end - counter).collect();
        if val.is_empty() {
            break;
        }
        let val = stringify(val);
        values.push(val);
        counter = range.end
    }
    values
}

fn process_input(file: impl BufRead, extract: &Extract) -> MyResult<()> {
    match extract {
        Extract::Bytes(ranges) => {
            for line in file.lines() {
                let line = line?;
                let mut bytes = line.bytes();
                let values: Vec<String> = process_ranges(bytes.by_ref(), ranges, |b| {
                    String::from_utf8_lossy(&b).to_string()
                });
                println!("{}", values.join(""));
            }
        }
        Extract::Chars(ranges) => {
            for line in file.lines() {
                let line = line?;
                let mut chars = line.chars();
                let values: Vec<String> =
                    process_ranges(chars.by_ref(), ranges, |c| c.into_iter().collect());
                println!("{}", values.join(""));
            }
        }
        Extract::Fields(ranges, delim) => {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(*delim as u8)
                .from_reader(file);
            let delim = format!("{delim}");
            for record in rdr.records() {
                let record = record?;
                let mut fields = record.into_iter();
                let values: Vec<String> =
                    process_ranges(fields.by_ref(), ranges, |c| c.join(&delim));
                println!("{}", values.join(&format!("{delim}")));
            }
        }
    }
    Ok(())
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    for file in &args.files {
        let file = shared_utils::open(&Some(file.as_str()));
        if let Err(error) = &file {
            eprintln!("{error}");
            continue;
        }
        let file = file.unwrap();
        process_input(file, &args.extract)?;
    }
    Ok(())
}
