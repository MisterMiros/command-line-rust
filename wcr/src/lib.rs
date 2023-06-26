use clap::Parser;
use shared_utils::MyResult;

#[derive(Debug, Parser)]
struct Args {
    #[arg(required = true)]
    files: Vec<String>,
}

pub fn run() -> MyResult<()> {
    
    Ok(())
}