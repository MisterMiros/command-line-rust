use std::{
    borrow::BorrowMut,
    error::Error,
    fs::{self},
    path::PathBuf,
};

use clap::Parser;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use regex::{Regex, RegexBuilder};
use shared_utils::MyResult;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct RawArgs {
    /// List of source files for the input
    #[arg(required = true)]
    sources: Vec<String>,

    /// A pattern to match against
    #[arg(short = 'm', long = "pattern")]
    pattern: Option<String>,

    /// Random seed
    #[arg(short, long)]
    seed: Option<u64>,

    /// Case insensitive pattern matching
    #[arg(short, long, default_value_t = false)]
    insensitive: bool,
}

struct Args {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

#[derive(Debug)]
struct Fortune {
    source: String,
    text: String,
}

impl Fortune {
    fn new(source: &PathBuf, text: &str) -> Fortune {
        Fortune {
            source: source
                .file_name()
                .map(|s| String::from(s.to_string_lossy()))
                .unwrap_or_default(),
            text: text.trim().to_string(),
        }
    }
}

fn get_args() -> MyResult<Args> {
    let args = RawArgs::try_parse()?;
    let pattern = if let Some(s) = args.pattern {
        Some(
            RegexBuilder::new(&s)
                .case_insensitive(args.insensitive)
                .build()?,
        )
    } else {
        None
    };
    Ok(Args {
        sources: args.sources,
        pattern: pattern,
        seed: args.seed,
    })
}

pub fn run() -> MyResult<()> {
    let args = get_args()?;
    let files = find_files(&args.sources)?;
    let fortunes = read_fortunes(&files)?;
    if let Some(pattern) = args.pattern {
        let matched: Vec<_> = fortunes
            .iter()
            .filter(|f| pattern.is_match(&f.text))
            .collect();
        let mut source = &String::new();
        for fortune in matched {
            if *source != *fortune.source {
                source = &(fortune.source);
                eprintln!("({})", source);
                eprintln!("%");
            }
            println!("{}", fortune.text);
            println!("%");
        }
    } else {
        if let Some(text) = pick_fortune(&fortunes, &args.seed) {
            println!("{}", text);
        } else {
            println!("No fortunes found");
        }
    }

    Ok(())
}

fn pick_fortune(fortunes: &[Fortune], seed: &Option<u64>) -> Option<String> {
    let mut rng = if let Some(seed) = seed {
        StdRng::seed_from_u64(*seed)
    } else {
        StdRng::from_entropy()
    };
    fortunes.choose(rng.borrow_mut()).map(|f| f.text.clone())
}

fn read_fortunes(paths: &[PathBuf]) -> MyResult<Vec<Fortune>> {
    let mut result: Vec<Fortune> = Vec::with_capacity(paths.len());
    for path in paths {
        let mut fortunes = read_fortunes_from_file(path)?;
        result.append(&mut fortunes);
    }
    Ok(result)
}

fn read_fortunes_from_file(path: &PathBuf) -> MyResult<Vec<Fortune>> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("{}: {}", path.as_os_str().to_string_lossy(), e))?;
    let fortunes: Vec<Fortune> = contents
        .split("%")
        .filter(|f| !f.trim().is_empty())
        .map(|f| Fortune::new(path, f))
        .collect();
    Ok(fortunes)
}

fn find_files(paths: &[String]) -> MyResult<Vec<PathBuf>> {
    let flat_sort_dedup = |p: Vec<Vec<_>>| {
        let mut p = p.concat();
        p.sort();
        p.dedup();
        p
    };

    paths
        .iter()
        .map(find_files_in_path)
        .collect::<Result<Vec<_>, _>>()
        .map(flat_sort_dedup)
        .map_err(From::from)
}

fn is_not_dat(path: &PathBuf) -> bool {
    match path.extension() {
        None => true,
        Some(s) => s != "dat",
    }
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

fn find_files_in_path(path: &String) -> MyResult<Vec<PathBuf>> {
    let dir = WalkDir::new(path);

    let file_filter = |e: &Result<DirEntry, _>| e.is_err() || e.as_ref().is_ok_and(is_file);
    let dat_filter = |e: &Result<PathBuf, _>| e.is_err() || e.as_ref().is_ok_and(is_not_dat);

    dir.into_iter()
        .filter(file_filter)
        .map(|e| e.map(|e| e.into_path()))
        .filter(dat_filter)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| {
            From::from(format!(
                "{}: {}",
                path,
                err.io_error()
                    .map(|err| err.to_string())
                    .unwrap_or(err.to_string())
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );
        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());
        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());
        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));
        // Test for multiple sources, path must be unique and sorted
        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }

    #[test]
    fn test_read_fortunes() {
        // One input file
        let res = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
        assert!(res.is_ok());
        if let Ok(fortunes) = res {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 6);
            assert_eq!(
                fortunes.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
 A. Collared greens."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\n\
 A: A bad idea (bad-eye deer)."
            );
        }
        // Multiple input files
        let res = read_fortunes(&[
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 11);
    }

    #[test]
    fn test_pick_fortune() {
        // Create a slice of fortunes
        let fortunes = &[
            Fortune {
                source: "fortunes".to_string(),
                text: "You cannot achieve the impossible without \
    attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Assumption is the mother of all screw-ups.".to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];
        // Pick a fortune with a seed
        assert_eq!(
            pick_fortune(fortunes, &Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
