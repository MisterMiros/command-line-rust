use assert_cmd::Command;
use path_macro::path;
use predicates::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;
use std::path::{Path, PathBuf};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "catr";
const ERROR: &str = "error.out";
const EMPTY: &str = "empty.txt";
const FOX: &str = "fox.txt";
const SPIDERS: &str = "spiders.txt";
const BUSTLE: &str = "the-bustle.txt";
const ALL: &str = "all";

macro_rules! out {
    ($str:tt) => {
        format!("{}.out", $str)
    };
}

macro_rules! out_n {
    ($str:tt) => {
        format!("{}.n.out", $str)
    };
}

macro_rules! out_b {
    ($str:tt) => {
        format!("{}.b.out", $str)
    };
}

macro_rules! out_stdin {
    ($str:tt) => {
        format!("{}.stdin.out", $str)
    };
}

macro_rules! out_stdin_n {
    ($str:tt) => {
        format!("{}.stdin.n.out", $str)
    };
}

macro_rules! out_stdin_b {
    ($str:tt) => {
        format!("{}.stdin.b.out", $str)
    };
}

macro_rules! expected {
    ($str:tt) => {
        path!("tests" / "expected" / $str)
    };
}

macro_rules! input {
    ($str:tt) => {
        path!("tests" / "inputs" / $str)
    };
}

#[test]
fn dies_no_args() -> TestResult {
    let path = expected!(ERROR);
    let expected = fs::read_to_string(&path)?;
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.assert().failure().stderr(expected);
    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = bad_file();
    let expected = format!("{}:.*\\(os error 2\\)", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], &[], &out!(EMPTY))
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], &[], &out!(SPIDERS))
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], &[], &out!(FOX))
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], &[], &out!(BUSTLE))
}

#[test]
fn all() -> TestResult {
    run(&[SPIDERS, FOX, EMPTY, BUSTLE], &[], &out!(ALL))
}

#[test]
fn empty_number_lines() -> TestResult {
    run(&[EMPTY], &["-n"], &out_n!(EMPTY))
}

#[test]
fn spiders_number_lines() -> TestResult {
    run(&[SPIDERS], &["-n"], &out_n!(SPIDERS))
}

#[test]
fn fox_number_lines() -> TestResult {
    run(&[FOX], &["-n"], &out_n!(FOX))
}

#[test]
fn bustle_number_lines() -> TestResult {
    run(&[BUSTLE], &["-n"], &out_n!(BUSTLE))
}

#[test]
fn all_number_lines() -> TestResult {
    run(&[SPIDERS, FOX, EMPTY, BUSTLE], &["-n"], &out_n!(ALL))
}

#[test]
fn empty_number_non_blank_lines() -> TestResult {
    run(&[EMPTY], &["-b"], &out_b!(EMPTY))
}

#[test]
fn spiders_number_non_blank_lines() -> TestResult {
    run(&[SPIDERS], &["-b"], &out_b!(SPIDERS))
}

#[test]
fn fox_number_non_blank_lines() -> TestResult {
    run(&[FOX], &["-b"], &out_b!(FOX))
}

#[test]
fn bustle_number_non_blank_lines() -> TestResult {
    run(&[BUSTLE], &["-b"], &out_b!(BUSTLE))
}

#[test]
fn all_number_non_blank_lines() -> TestResult {
    run(&[SPIDERS, FOX, EMPTY, BUSTLE], &["-b"], &out_b!(ALL))
}


#[test]
fn stdin() -> TestResult {
    run_stdin(BUSTLE, &[], &out_stdin!(BUSTLE))
}

#[test]
fn stdin_number_lines() -> TestResult {
    run_stdin(BUSTLE, &["-n"], &out_stdin_n!(BUSTLE))
}

#[test]
fn stdin_number_non_blank_lines() -> TestResult {
    run_stdin(BUSTLE, &["-b"], &out_stdin_b!(BUSTLE))
}

fn run(files: &[&str], args: &[&str], expected_file: &str) -> TestResult {
    let expected_file = expected!(expected_file);
    let expected = fs::read_to_string(&expected_file)?;

    let files: Vec<PathBuf> = files.iter().map(|x| input!(x)).collect();
    Command::cargo_bin(PRG)?
        .args(files)
        .args(args)
        .assert()
        .stdout(expected);
    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let expected_file = expected!(expected_file);
    let expected = fs::read_to_string(&expected_file)?;

    let input_file = input!(input_file);
    Command::cargo_bin(PRG)?
        .arg("-")
        .args(args)
        .pipe_stdin(input_file)?
        .assert()
        .stdout(expected);
    Ok(())
}

fn bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}
