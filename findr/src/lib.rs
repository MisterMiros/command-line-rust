use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use regex::Regex;
use shared_utils::MyResult;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `find`
pub struct Args {
    /// Search path(s)
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Names
    #[arg(
        short('n'),
        long("name"),
        value_name = "NAME",
        value_parser(Regex::new),
        action(ArgAction::Append),
        num_args(0..)
    )]
    names: Vec<Regex>,

    /// Entry types
    #[arg(
        short('t'),
        long("type"),
        value_name = "TYPE",
        value_parser(clap::value_parser!(EntryType)),
        action(ArgAction::Append),
        num_args(0..)
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

fn process_path(path: &str, args: &Args) -> MyResult<()> {
    for entry in WalkDir::new(path) {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(entry) => {
                let ft = entry.file_type();
                if ft.is_dir() && !args.entry_types.contains(&EntryType::Dir) {
                    continue;
                } 
                if ft.is_file() && !args.entry_types.contains(&EntryType::File) {
                    continue;
                } 
                if ft.is_symlink() && !args.entry_types.contains(&EntryType::Link) {
                    continue;
                }
                let file_name = entry.file_name().to_string_lossy();
                if !args.names.is_empty() && !args.names.iter().any(|n| n.is_match(&file_name)) {
                    continue;
                }
                println!("{}", entry.path().display())
            }
        }
    }
    Ok(())
}

fn get_args() -> Args {
    let mut args = Args::parse();
    if args.entry_types.is_empty() {
        args.entry_types = Vec::from(EntryType::value_variants());
    }
    args
}

pub fn run() -> MyResult<()>{
    let args = get_args();
    for path in &args.paths {
        process_path(path, &args)?;
    }
    Ok(())
}
