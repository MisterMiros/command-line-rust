use std::fs;
use std::io::BufRead;
use std::iter;

use clap::Parser;
use regex::Regex;
use shared_utils::MyResult;
use walkdir::WalkDir;

macro_rules! print_match {
    ($result: ident, $filename: ident) => {
        match $filename {
            Some(filename) => println!("{}: {}", filename, $result),
            None => println!("{}", $result),
        }
    };
}

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
    ignore_case: bool,
}

struct Args {
    pattern: Regex,
    paths: Vec<String>,
    count: bool,
    invert_match: bool,
    recursive: bool,
    ignore_case: bool,
}

fn get_args() -> MyResult<Args> {
    let args = RawArgs::try_parse()?;
    let pattern = regex::RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()?;
    Ok(Args {
        paths: args.paths,
        count: args.count,
        invert_match: args.invert_match,
        recursive: args.recursive,
        ignore_case: args.ignore_case,
        pattern,
    })
}

fn process_file(file: impl BufRead, args: &Args, filename: &Option<&str>) -> MyResult<()> {
    let mut count = 0;
    for line in file.lines() {
        let line = line?;
        match (args.invert_match, args.pattern.is_match(&line)) {
            (true, false) | (false, true) => {
                count += 1;
                if !args.count {
                    print_match!(line, filename);
                }
            }
            _ => ()
        }
    }
    if args.count {
        print_match!(count, filename);
    }
    Ok(())
}

fn process_path(path: &str, recursive: bool) -> Box<dyn Iterator<Item = MyResult<String>>> {
    if path == "-" {
        return Box::from(iter::once(Ok(String::from(path))));
    }
    let file_metadata = fs::metadata(path);
    if let Err(error) = file_metadata {
        return Box::from(iter::once(Err(From::from(error))));
    }
    let file_metadata = file_metadata.unwrap();
    let is_dir = file_metadata.file_type().is_dir();
    if is_dir && !recursive {
        let error = format!("{path}: Is a directory");
        return Box::from(iter::once(Err(From::from(error))));
    }
    if is_dir {
        let file_tree = WalkDir::new(path);
        let paths = file_tree.into_iter().filter_map(move |f| match f {
            Ok(file) => {
                if file.file_type().is_dir() {
                    None
                } else {
                    Some(Ok(String::from(file.path().to_string_lossy().into_owned())))
                }
            }
            Err(error) => Some(Err(From::from(error))),
        });
        return Box::from(paths);
    }
    Box::from(iter::once(Ok(String::from(path))))
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    let print_filenames = args.paths.len() > 1;
    for path in &args.paths {
        let filenames = process_path(path, args.recursive);
        filenames.map(|filename| {
            let filename = filename?;
            let file = shared_utils::open(&Some(&filename))?;
            let filename = if print_filenames { Some(filename.as_str()) } else { None };
            process_file(file, &args, &filename)?;
            Ok(())
        }).for_each(|r: MyResult<()>| {
            if let Err(error) = r {
                eprintln!("{error}");
            }
        });
    }
    Ok(())
}
