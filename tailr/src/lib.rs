use std::{
    collections::VecDeque,
    io::{BufRead, Read},
};

use clap::Parser;
use shared_utils::MyResult;

enum Mode {
    Bytes(usize),
    Lines(usize),
    BytesFrom(usize),
    LinesFrom(usize),
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct RawArgs {
    /// Files to print the lines from
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number of bytes to print
    #[arg(short = 'c', long, conflicts_with = "lines")]
    bytes: Option<String>,

    /// Number of lines to print
    #[arg(short = 'n', long, default_value = "10", conflicts_with = "bytes")]
    lines: String,

    /// Suppresses printing of headers when multiple files are being examined.
    #[arg(short, long)]
    quiet: bool,
}

struct Args {
    files: Vec<String>,
    mode: Mode,
    quiet: bool,
}

fn get_args() -> MyResult<Args> {
    let args = RawArgs::try_parse()?;

    //println!("{args:?}");
    let mode = if let Some(value) = args.bytes {
        let count: usize = value.replace("-", "").parse()
            .map_err(|_| format!("illegal byte count -- {value}"))?;
        if value.starts_with("+") {
            Mode::BytesFrom(count)
        } else {
            Mode::Bytes(count)
        }
    } else {
        let count: usize = args.lines.replace("-", "").parse()
            .map_err(|_| format!("illegal line count -- {}", args.lines))?;
        if args.lines.starts_with("+") {
            Mode::LinesFrom(count)
        } else {
            Mode::Lines(count)
        }
    };

    Ok(Args {
        files: args.files,
        mode,
        quiet: args.quiet,
    })
}

fn process_file(filename: &str, file: Box<dyn BufRead>, mode: &Mode) -> MyResult<()> {
    match mode {
        Mode::BytesFrom(index) => {
            let iter = file.bytes();
            let bytes = (iter.skip((*index).saturating_sub(1usize)).collect::<Result<Vec<u8>, _>>())?;
            let str = String::from_utf8_lossy(&bytes).to_string();
            if !str.is_empty() {
                print!("{str}");
            }
        }
        Mode::LinesFrom(index) => {
            let iter = file.lines();
            let lines = (iter.skip((*index).saturating_sub(1usize)).collect::<Result<Vec<String>, _>>())?;
            for line in lines {
                println!("{line}");
            }
        }
        Mode::Bytes(count) => {
            let file_size = std::fs::metadata(filename).unwrap().len() as usize;
            let skip = file_size.saturating_sub(*count);
            let iter = file.bytes();
            let bytes = (iter.skip(skip).collect::<Result<Vec<u8>, _>>())?;
            let str = String::from_utf8_lossy(&bytes).to_string();
            if !str.is_empty() {
                print!("{str}");
            }
        }
        Mode::Lines(count) => {
            if *count == 0 { 
                return Ok(());
            }
            let mut iter = file.lines();
            let lines = iter.try_fold::<_, _, MyResult<_>>(
                VecDeque::with_capacity(*count),
                |mut acc, line| {
                    let line = line?;
                    if acc.len() == *count {
                        acc.pop_front();
                    }
                    acc.push_back(line);
                    Ok(acc)
                },
            )?;
            for line in lines {
                println!("{line}");
            }
        }
    }
    Ok(())
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    let total_files = args.files.len();
    let print_names = !args.quiet && total_files > 1;
    for (num, filename) in args.files.iter().enumerate() {
        if print_names {
            println!("==> {filename} <==");
        }
        let file = shared_utils::open(&Some(&filename));
        if let Err(error) = &file {
            println!("{error}");
        }
        let file = file.unwrap();
        if let Err(error) = process_file(&filename, file, &args.mode) {
            println!("{filename}: {error}");
        }
        let is_last = num + 1 == total_files;
        if print_names && !is_last {
            println!();
        }
    }
    Ok(())
}