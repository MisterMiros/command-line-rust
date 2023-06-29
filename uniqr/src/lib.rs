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

pub fn run() -> MyResult<()> {
    Ok(())
}