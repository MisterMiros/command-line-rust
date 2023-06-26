use std::{io::BufRead, fmt::format};

use clap::Parser;
use shared_utils::MyResult;

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    pub num_lines: usize,
    pub num_words: usize,
    pub num_bytes: usize,
    pub num_chars: usize,
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(required = true)]
    files: Vec<String>,

    #[arg(short = 'c', long, conflicts_with = "chars")]
    bytes: bool,

    #[arg(short = 'm', long)]
    chars: bool,

    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    words: bool,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let num_bytes = content.bytes().count();
    let num_chars = content.chars().count();
    let num_words = content.split_whitespace().count();
    let num_lines = content.lines().count();
    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn get_args() -> MyResult<Args> {
    let mut args = Args::try_parse()?;
    if ![args.lines, args.words, args.chars, args.bytes].iter().any(|f| *f) {
        args.lines = true;
        args.words = true;
        args.chars = true;
    }
    Ok(args)
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    for filename in args.files {
        let file = shared_utils::open(&filename);
        if let Err(error) = file {
            println!("Failed to open file {filename}. Error: {error}");
            continue;
        }
        let info = count(file.unwrap());
        if let Err(error) = info {
            println!("Failed get file info {filename}. Error: {error}");
            continue;
        }
        let info = info.unwrap();
        let mut result = String::new();
        if args.lines {
            result += &format!("\t{}", info.num_lines);
        }
        if args.words {
            result += &format!("\t{}", info.num_words);
        }
        if args.bytes {
            result += &format!("\t{}", info.num_bytes);
        }
        if args.chars {
            result += &format!("\t{}", info.num_chars);
        }
        println!("{result}\t{filename}");
    }
    Ok(())
}
