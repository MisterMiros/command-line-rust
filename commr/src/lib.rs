use std::{cmp::Ordering, io::BufRead};

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

macro_rules! print_first {
    ($value: ident, $args: ident) => {
        if !$args.suppress_first {
            println!("{}", $value);
        }
    };
}

macro_rules! print_second {
    ($value: ident, $args: ident) => {
        if !$args.suppress_second {
            if ($args.suppress_first) {
                println!("{}", $value);
            } else {
                println!("{}{}", $args.delimiter, $value);
            }
        }
    };
}

macro_rules! print_common {
    ($value: ident, $args: ident) => {
        if !$args.suppress_common {
            if $args.suppress_first && $args.suppress_second {
                println!("{}", $value);
            } else if $args.suppress_first || $args.suppress_second {
                println!("{}{}", $args.delimiter, $value);
            } else {
                println!("{0}{0}{1}", $args.delimiter, $value);
            }
        }
    };
}

fn process_files(file1: impl BufRead, file2: impl BufRead, args: &Args) -> MyResult<()> {
    let case = |s: String| {
        if args.insensitive {
            s.to_lowercase()
        } else {
            s
        }
    };
    let mut lines1 = file1.lines().map(|r| r.map(case));
    let mut lines2 = file2.lines().map(|r| r.map(case));
    let mut line_pair = (lines1.next(), lines2.next());

    loop {
        line_pair = match line_pair {
            (None, None) => break,
            (Some(Err(error)), _) | (_, Some(Err(error))) => return Err(From::from(error)),
            (Some(Ok(str1)), None) => {
                print_first!(str1, args);
                (lines1.next(), None)
            }
            (None, Some(Ok(str2))) => {
                print_second!(str2, args);
                (None, lines2.next())
            }
            (Some(Ok(str1)), Some(Ok(str2))) => match str1.cmp(&str2) {
                Ordering::Greater => {
                    print_second!(str2, args);
                    (Some(Ok(str1)), lines2.next())
                }
                Ordering::Less => {
                    print_first!(str1, args);
                    (lines1.next(), Some(Ok(str2)))
                }
                Ordering::Equal => {
                    print_common!(str1, args);
                    (lines1.next(), lines2.next())
                }
            },
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
