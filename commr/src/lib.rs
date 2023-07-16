use std::io::BufRead;

use clap::Parser;
use shared_utils::MyResult;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    file1: String,
    file2: Option<String>,

    /// Suppress printing of column 1
    #[arg(short = '1')]
    suppress_first: bool,

    /// Suppress printing of column 2
    #[arg(short = '2')]
    suppress_second: bool,

    /// Suppress printing of column 3
    #[arg(short = '3')]
    suppress_common: bool,

    /// Case insensitive comparison
    #[arg(short, long)]
    insensitive: bool,

    /// Columns delimiter
    #[arg(short, long = "output-delimiter", default_value = "\t")]
    delimiter: String,
}

fn get_args() -> MyResult<Args> {
    let args = Args::try_parse()?;
    let file1_stdin = args.file1 == "-";
    let file2_stdin = args.file2.is_none() || args.file2.as_ref().is_some_and(|s| s == "-");
    if file1_stdin && file2_stdin {
        return Err(From::from("Both input files cannot be STDIN"));
    }
    Ok(args)
}

fn process_files(file1: impl BufRead, file2: impl BufRead, args: &Args) -> MyResult<()> {
    let mut lines1 = file1.lines();
    let mut lines2 = file2.lines();
    let mut line_pair = (lines1.next(), lines2.next());
    loop {
        line_pair = match line_pair {
            (None, None) => break,
            (Some(Err(error)), _) | (_, Some(Err(error))) => return Err(From::from(error)),
            (Some(Ok(str1)), None) => {
                println!("{str1}{0}{0}", args.delimiter);
                (lines1.next(), None)
            }
            (None, Some(Ok(str2))) => {
                println!("{0}{str2}{0}", args.delimiter);
                (None, lines2.next())
            }
            (Some(Ok(str1)), Some(Ok(str2))) if str1 < str2 => {
                println!("{str1}{0}{0}", args.delimiter);
                (lines1.next(), Some(Ok(str2)))
            }
            (Some(Ok(str1)), Some(Ok(str2))) if str1 > str2 => {
                println!("{0}{str2}{0}", args.delimiter);
                (Some(Ok(str1)), lines2.next())
            }
            (Some(Ok(str1)), Some(Ok(_))) => {
                println!("{0}{0}{str1}", args.delimiter);
                (lines1.next(), lines2.next())
            }
        }
    }

    Ok(())
}

macro_rules! as_str {
    ($option: expr) => {
        $option.as_ref().map(|s| s.as_str())
    };
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    let file1 = shared_utils::open(&Some(args.file1.as_str()))?;
    let file2 = shared_utils::open(&as_str!(args.file2))?;
    process_files(file1, file2, &args)?;
    Ok(())
}
