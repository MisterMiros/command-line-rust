use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn open(filename: &Option<&str>) -> MyResult<Box<dyn BufRead>> {
    match filename {
        None | Some("-") => Ok(Box::new(BufReader::new(io::stdin()))),
        Some(filename) => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}
