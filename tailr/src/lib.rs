use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

use clap::Parser;
use shared_utils::MyResult;

enum Mode {
    Bytes(usize),
    BytesFrom(usize),
    Lines(usize),
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
        let count = value
            .replace("-", "")
            .parse()
            .map_err(|_| format!("illegal byte count -- {value}"))?;
        if value.starts_with("+") {
            Mode::BytesFrom(count)
        } else {
            Mode::Bytes(count)
        }
    } else {
        let count = args
            .lines
            .replace("-", "")
            .parse()
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

fn count_lines_and_bytes(filename: &str) -> MyResult<(usize, usize)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut num_lines = 0usize;
    let mut num_bytes = 0usize;
    let mut buf = Vec::new();
    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        num_bytes += bytes_read;
        num_lines += 1;
    }
    Ok((num_lines, num_bytes))
}

fn process_file(filename: &str, mode: &Mode) -> MyResult<()> {
    let (total_lines, total_bytes) = count_lines_and_bytes(filename)?;
    let file = File::open(filename)?;
    match mode {
        Mode::BytesFrom(index) => {
            process_bytes(file, index.saturating_sub(1))?;
        }
        Mode::Bytes(count) => {
            process_bytes(file, total_bytes.saturating_sub(*count))?;
        }
        Mode::LinesFrom(index) => {
            let file = BufReader::new(file);
            process_lines(file, *index)?;
        }
        Mode::Lines(count) => {
            if *count == 0 {
                return Ok(());
            }
            let file = BufReader::new(file);
            process_lines(file, total_lines.saturating_sub(*count) + 1)?;
        }
    }
    Ok(())
}

fn process_bytes(mut file: impl Read + Seek, from: usize) -> MyResult<()> {
    file.seek(SeekFrom::Start(from as u64))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    print!("{}", String::from_utf8_lossy(&buf));
    Ok(())
}

fn process_lines(mut file: impl BufRead, from: usize) -> MyResult<()> {
    let mut current = 0usize;
    let mut buf = String::new();
    while file.read_line(&mut buf)? > 0 {
        current += 1;
        if current >= from {
            print!("{buf}");
        }
        buf.clear();
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
        if let Err(error) = process_file(&filename, &args.mode) {
            eprintln!("{filename}: {error}");
        }
        let is_last = num + 1 == total_files;
        if print_names && !is_last {
            println!();
        }
    }
    Ok(())
}
