use std::{io::BufRead, ops::AddAssign};

use clap::Parser;
use shared_utils::MyResult;

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    pub name: Option<String>,
    pub num_lines: usize,
    pub num_words: usize,
    pub num_bytes: usize,
    pub num_chars: usize,
}

impl FileInfo {
    fn new(name: &Option<&str>) -> Self {
        FileInfo { 
            name: match name {
                None => None,
                Some(str) => Some(String::from(*str)),
            }, 
            num_lines: 0, 
            num_words: 0, 
            num_bytes: 0, 
            num_chars: 0 
        }
    }
}

impl AddAssign<&FileInfo> for FileInfo {
    fn add_assign(&mut self, rhs: &FileInfo) {
        self.num_bytes += rhs.num_bytes;
        self.num_chars += rhs.num_chars;
        self.num_lines += rhs.num_lines;
        self.num_words += rhs.num_words;
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg()]
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

pub fn count(filename: &Option<&str>, mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut info = FileInfo::new(filename);
    let mut buf = String::new();
    loop {
        let read = file.read_line(&mut buf)?;
        if read == 0 {
            break;
        }
        info.num_bytes += read;
        info.num_lines += 1;
        info.num_words += buf.split_whitespace().count();
        info.num_chars += buf.chars().count();
        buf.clear();
    }
    Ok(info)
}

fn get_args() -> MyResult<Args> {
    let mut args = Args::try_parse()?;
    if ![args.lines, args.words, args.chars, args.bytes].iter().any(|f| *f) {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    Ok(args)
}

fn print_info(info: &FileInfo, args: &Args, width: usize) {
    let mut space = "";
    if args.lines {
        print!("{space}{:>width$}", info.num_lines);
        space = " ";
    }
    if args.words {
        print!("{space}{:>width$}", info.num_words);
        space = " ";
    }
    if args.bytes {
        print!("{space}{:>width$}", info.num_bytes);
        space = " ";
    }
    if args.chars {
        print!("{space}{:>width$}", info.num_chars);
    }
    match &info.name {
        Some(name) => println!(" {}", name),
        None => println!()
    };
}

fn get_width(total: &FileInfo, args: &Args) -> usize {
    if [args.lines, args.words, args.chars, args.bytes].iter().filter(|f| **f).count() == 1 {
        return 1;
    }
    if total.num_bytes > 9 {
        1 + total.num_bytes.ilog10() as usize
    } else {
        1
    }
}

fn format_error(filename: &Option<&str>, error: &Box<dyn std::error::Error>) -> String {
    match filename {
        None => format!("wcr: {error}"),
        Some(filename) => format!("wcr: {filename}: {error}"),
    }
}

fn process_file(filename: &Option<&str>) -> Result<FileInfo, String> {
    let file = shared_utils::open(filename);
    if let Err(error) = file {
        return Err(format_error(filename, &error));
    }
    let info = count(filename, file.unwrap());
    if let Err(error) = info {
        return Err(format_error(filename, &error));
    }
    Ok(info.unwrap())
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    let mut results: Vec<Result<FileInfo, String>> = Vec::with_capacity(args.files.len());
    let mut total = FileInfo::new(&Some("total"));
    if args.files.is_empty() {
        let result = process_file(&None);
        if let Ok(info) = &result {
            total += info;
        }
        results.push(Ok(result.unwrap()));
    } else {
        for filename in &args.files {
            let result = process_file(&Some(filename));
            if let Ok(info) = &result {
                total += info;
            }
            results.push(Ok(result.unwrap()));
        }
    }

    let width = get_width(&total, &args);
    for result in results {
        match result {
            Err(error) => println!("{error}"),
            Ok(info) => print_info(&info, &args, width),
        };
    }
    if args.files.len() > 1 {
        print_info(&total, &args, width)
    }
    Ok(())
}
