use std::{io::{BufRead, BufReader, self}, fs::File};

pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
