use std::{error::Error, io::{BufRead, self, BufReader}, fs::File};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Files to concatenate
    #[arg(required = true)]
    files: Vec<String>,

    /// Number the lines in concatenated output
    #[arg(short = 'n', long = "number")]
    number_lines: bool,

    /// Number only nonblank lines in concatenate output
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

enum Counter {
    All { count: u32 },
    NonBlank { count: u32 },
    None,
}

impl From<&Args> for Counter {
    fn from(args: &Args) -> Self {
        if args.number_lines {
            Self::All { count: 1 }
        } else if args.number_nonblank_lines {
            Self::NonBlank { count: 1 }
        } else {
            Self::None
        }
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_file(buf: Box<dyn BufRead>, counter: &mut Counter) {
    for line in buf.lines() {
        match line {
            Ok(s) => print_line(&s, counter),
            Err(error) => println!("Failed to read the line: {error}"),
        }
    }
}

fn print_line(line: &str, counter: &mut Counter) {
    match counter {
        Counter::None => println!("{line}"),
        Counter::All { count } => {
            println!("{:>6}\t{line}", count);
            *count += 1
        },
        Counter::NonBlank { count } =>  {
            if !line.is_empty() {
                println!("{:>6}\t{line}", count);
                *count += 1
            } else {
                println!()
            }
        }
    }
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let mut counter = Counter::from(&args);
    for filename in &args.files {
        let file = open(&filename);
        match file {
            Ok(buf) => print_file(buf, &mut counter),
            Err(error) => println!("Failed to open {filename}: {error}"),
        }
    }
    Ok(())
}