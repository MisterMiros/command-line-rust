use std::fs;
use std::io::BufRead;

use clap::Parser;
use regex::{Regex, RegexBuilder};
use shared_utils::MyResult;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct RawArgs {
    /// RegEx pattern to search for
    #[arg(required = true)]
    pattern: String,

    /// Paths to search in
    #[arg(default_value = "-")]
    paths: Vec<String>,

    /// Print count of matching lines in files
    #[arg(short, long)]
    count: bool,

    /// Search for lines that do not match the pattern
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// Recursive search through directories
    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    insensitive: bool,
}

struct Args {
    pattern: Regex,
    paths: Vec<String>,
    count: bool,
    invert_match: bool,
    recursive: bool,
}

fn get_args() -> MyResult<Args> {
    let args = RawArgs::try_parse()?;
    let result = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.insensitive)
        .build();
    match result {
        Err(_) => Err(From::from(format!("Invalid pattern \"{}\"", args.pattern))),
        Ok(pattern) => Ok(Args {
            paths: args.paths,
            count: args.count,
            invert_match: args.invert_match,
            recursive: args.recursive,
            pattern,
        }),
    }
}

macro_rules! print_match {
    ($result: ident, $filename: ident) => {
        match $filename {
            Some(filename) => println!("{}:{}", filename, $result),
            None => println!("{}", $result),
        }
    };
}

fn process_file(file: impl BufRead, args: &Args, filename: &Option<&str>) -> MyResult<()> {
    let mut count = 0;
    for line in file.lines() {
        let line = line?;
        let is_match = args.pattern.is_match(&line);
        if let (true, false) | (false, true) = (args.invert_match, is_match) {
            count += 1;
            if !args.count {
                print_match!(line, filename);
            }
        }
    }
    if args.count {
        print_match!(count, filename);
    }
    Ok(())
}

macro_rules! ok_once {
    ($value: expr) => {
        Box::from(std::iter::once(Ok(From::from($value))))
    };
}

macro_rules! err_once {
    ($value: expr) => {
        Box::from(std::iter::once(Err(From::from($value))))
    };
}

macro_rules! is_dir {
    ($file: ident) => {
        $file.file_type().is_dir()
    };
}

type DynIter<T> = Box<dyn Iterator<Item = T>>;

fn process_path(path: &str, recursive: bool) -> DynIter<MyResult<String>> {
    if path == "-" {
        return ok_once!(path);
    }

    let file_metadata = fs::metadata(path);
    if let Err(error) = file_metadata {
        return err_once!(format!("{path}: {error}"));
    }

    let file_metadata = file_metadata.unwrap();
    if !is_dir!(file_metadata) {
        return ok_once!(path);
    }

    if !recursive {
        return err_once!(format!("{path} is a directory"));
    }

    let file_tree = WalkDir::new(path);
    let paths = file_tree.into_iter().filter_map(move |f| match f {
        Ok(file) => {
            if is_dir!(file) {
                None
            } else {
                Some(Ok(String::from(file.path().to_string_lossy().into_owned())))
            }
        }
        Err(error) => Some(Err(From::from(error))),
    });
    Box::from(paths)
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    let filenames: Vec<_> = args
        .paths
        .iter()
        .flat_map(|p| process_path(p, args.recursive))
        .collect();
    let print_filenames = filenames.len() > 1;

    filenames
        .into_iter()
        .map(|filename| {
            let filename = filename?;
            let file = shared_utils::open(&Some(&filename))?;
            let filename = if print_filenames {
                Some(filename.as_str())
            } else {
                None
            };
            process_file(file, &args, &filename)?;
            Ok(())
        })
        .for_each(|r: MyResult<()>| {
            if let Err(error) = r {
                eprintln!("{error}");
            }
        });
    Ok(())
}
