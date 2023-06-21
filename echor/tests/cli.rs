use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const COMMAND_NAME: &str = "echor";

#[test]
fn dies_no_args() -> TestResult {
    let path = std::path::Path::new("tests").join("expected").join("error.txt");
    let expected = fs::read_to_string(path)?;
    let mut cmd = Command::cargo_bin(COMMAND_NAME)?;
    cmd.assert()
        .failure()
        .stderr(expected);
    Ok(())
}

#[test]
fn one_arg_runs() -> TestResult {
    run(&["Hello World"], "onearg.txt")
}

#[test]
fn two_arg_runs() -> TestResult {
    run(&["Hello", "World"], "twoargs.txt")
}

#[test]
fn one_arg_omit_new_line_runs() -> TestResult {
    run(&["Hello World", "-n"], "onearg.n.txt")
}

#[test]
fn two_arg_omit_new_line_runs() -> TestResult {
    run(&["Hello", "World", "-n"], "twoargs.n.txt")
}

fn run(args: &[&str], expected_filename: &str) -> TestResult {
    let path = std::path::Path::new("tests").join("expected").join(expected_filename);
    let expected = fs::read_to_string(path)?;
    Command::cargo_bin(COMMAND_NAME)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
    Ok(())
}
