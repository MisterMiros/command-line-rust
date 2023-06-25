use std::io::{BufRead, self, BufReader};
use std::fs::File;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

enum Mode {
    Bytes(usize),
    Lines(usize),
}

impl From<&Args> for Mode {
    fn from(value: &Args) -> Self {
        if let Some(count) = value.bytes {
            Mode::Bytes(count)
        } else {
            Mode::Lines(value.lines)
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Files to print the lines from
    #[arg(required = true)]
    files: Vec<String>,

    /// Number of bytes to print
    #[arg(short = 'c', long, conflicts_with = "lines")]
    bytes: Option<usize>,

    /// Number of lines to print
    #[arg(short = 'n', long, default_value_t = 10, conflicts_with = "bytes")]
    lines: usize,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_file(mut file: Box<dyn BufRead>, mode: &Mode) -> MyResult<()> {
    match mode {
        Mode::Lines(lines) => {
            for line in file.lines().take(*lines) {
                let line = line?;
                println!("{line}")
            }
        },
        Mode::Bytes(bytes) => {
            let mut buf: Vec<u8> = vec![0; *bytes];            
            let bytes = file.read(&mut buf)?;
            let string = String::from_utf8_lossy(&buf[..bytes]);
            print!("{string}");
        }
    };
    Ok(())
}

pub fn run() -> MyResult<()> {
    let args = Args::try_parse()?;
    let mode = Mode::from(&args);
    let total_files = args.files.len();
    let print_names = total_files > 1;
    for (num, filename) in args.files.iter().enumerate() {
        if print_names {
            println!("==> {filename} <==");
        }
        let file = open(filename);        
        match file {
            Err(error) => println!("Failed to read file {filename}: {error}"),
            Ok(file) => {
                if let Err(error) = print_file(file, &mode) {
                    println!("Failed to print file {filename} contents: {error}");
                }
            }
        }
        let is_last = num + 1 == total_files;
        if print_names && !is_last {
            println!();
        }
    }
    Ok(())
}