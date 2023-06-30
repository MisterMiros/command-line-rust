use std::io::{BufRead, BufWriter, Write, self};
use std::fs::{ File, self };

use shared_utils::MyResult;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// A file to read lines from
    input_file: Option<String>,

    /// A file to write the results to
    output_file: Option<String>,

    #[arg(short, long)]
    /// Prefix lines by the number of occurrences
    count: bool,
}

fn write_to_output(line: &str, writer: &mut BufWriter<Box<dyn Write>>, print_count: bool, count: usize) -> MyResult<()> {
    if print_count {
        writeln!(writer, "{:>7} {}", count, line)?;
    } else {
        writeln!(writer, "{line}")?;
    }
    Ok(())
}

fn write_lines(input: Box<dyn BufRead>, output: &mut BufWriter<Box<dyn Write>>, print_count: bool) -> MyResult<()> {
    let mut lines = input.lines();
    let prev_line = lines.next();
    if prev_line.is_none() {
        return Ok(());
    }
    let mut prev_line = prev_line.unwrap()?;
    let mut count: usize = 1;
    for line in lines {
        let line = line?;
        if line == prev_line {
            count += 1;
            continue;
        }
        write_to_output(&prev_line, output, print_count, count)?;
        prev_line = line;
        count = 1;
    }
    if !prev_line.is_empty() {
        write_to_output(&prev_line, output, print_count, count)?;
    }
    
    Ok(())
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let input = shared_utils::open(&args.input_file.as_ref().map(|s| s.as_str()))?;
    let mut output: BufWriter<Box<dyn Write>> = match args.output_file.as_ref().map(|s| s.as_str()) {
        None | Some("-") => {
            BufWriter::new(Box::new(io::stdout()))
        },
        Some(filename) => {
            let output_file = match fs::metadata(filename) {
                Ok(_) => File::open(filename),
                Err(_) => File::create(filename),
            }?;
            BufWriter::new(Box::new(output_file))
        }
    };
    write_lines(input, &mut output, args.count)?;
    output.flush()?;

    Ok(())
}