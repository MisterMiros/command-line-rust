use chrono::{DateTime, Local};
use clap::Parser;
use shared_utils::MyResult;
use std::os::unix::prelude::MetadataExt;
use std::{fs, path::PathBuf};
use tabular::{Row, Table};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Directory paths to display contents of
    #[arg()]
    paths: Vec<String>,

    /// Display files in a long format
    #[arg(short, long)]
    long: bool,

    /// Show hidden files
    #[arg(short = 'a', long = "all")]
    show_hidden: bool,
}

fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut result = Vec::new();
    for path in paths {
        let metadata = fs::metadata(path);
        if let Err(err) = metadata {
            eprintln!("{path}: {err}");
            continue;
        }
        let metadata = metadata.unwrap();
        if metadata.is_file() {
            result.push(PathBuf::from(path));
            continue;
        }
        let mut files_in_dir = find_files_in_dir(path, show_hidden)?;
        result.append(&mut files_in_dir);
    }
    Ok(result)
}

fn find_files_in_dir(path: &str, show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut result = Vec::new();
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_name = String::from(entry.file_name().to_string_lossy());
        if !show_hidden && file_name.starts_with(".") {
            continue;
        }
        result.push(entry.path());
    }
    Ok(result)
}

fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    let fmt = "{:<}{:<} {:>} {:<} {:<} {:>} {:<} {:<}";
    let mut table = Table::new(fmt);
    for path in paths {
        let row = format_row(path)?;
        table.add_row(row);
    }
    Ok(format!("{}", table))
}

fn format_row(path: &PathBuf) -> MyResult<Row> {
    let metadata = path.metadata()?;

    let dir_cell = if metadata.is_dir() {
        String::from("d")
    } else {
        String::from("-")
    };

    let permissions_cell = format_mode(metadata.mode());

    let nlink_cell = format!("{}", metadata.nlink());

    let user_cell = users::get_user_by_uid(metadata.uid())
        .map_or(String::new(), |u| String::from(u.name().to_string_lossy()));

    let group_cell = users::get_group_by_gid(metadata.gid())
        .map_or(String::new(), |g| String::from(g.name().to_string_lossy()));

    let size_cell = format!("{}", metadata.size());

    let modified: DateTime<Local> = metadata.modified()?.into();
    let modified_cell = modified.to_string();

    Ok(Row::new()
        .with_cell(dir_cell) // 1 "d" or "-"
        .with_cell(permissions_cell) // 2 permissions
        .with_cell(nlink_cell) // 3 number of links
        .with_cell(user_cell) // 4 user name
        .with_cell(group_cell) // 5 group name
        .with_cell(size_cell) // 6 size
        .with_cell(modified_cell) // 7 modification
        .with_cell(path.to_str().unwrap()))
}

fn format_mode(mode: u32) -> String {
    let mut result = String::from("---------");
    // other
    if mode & 0o001 != 0 {
        result.replace_range(8..9, "x");
    }
    if mode & 0o002 != 0 {
        result.replace_range(7..8, "w");
    }
    if mode & 0o004 != 0 {
        result.replace_range(6..7, "r");
    }

    if mode & 0o010 != 0 {
        result.replace_range(5..6, "x");
    }
    if mode & 0o020 != 0 {
        result.replace_range(4..5, "w");
    }
    if mode & 0o040 != 0 {
        result.replace_range(3..4, "r");
    }

    if mode & 0o100 != 0 {
        result.replace_range(2..3, "x");
    }
    if mode & 0o200 != 0 {
        result.replace_range(1..2, "w");
    }
    if mode & 0o400 != 0 {
        result.replace_range(0..1, "r");
    }
    result
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let paths = if args.paths.is_empty() {
        vec![String::from("./")]
    } else {
        args.paths
    };
    let files = find_files(paths.as_slice(), args.show_hidden)?;
    if args.long {
        let formatted = format_output(files.as_slice())?;
        println!("{}", formatted);
    } else {
        for file in files {
            println!("{}", file.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{ find_files, format_mode };
    #[test]
    fn test_find_files() {
        // Find all nonhidden entries in a directory
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
        // Find all entries in a directory
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
        // Any existing file should be found even if hidden
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);
        // Test multiple path arguments
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }
}
