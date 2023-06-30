use assert_cmd::Command;
use path_macro::path;
use predicates::str;
use std::{fs, path::PathBuf};
use test_case::test_case;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "uniqr";

macro_rules! expected {
    ($str: tt) => {
        path!("tests" / "expected" / $str)
    };
}

macro_rules! input {
    ($str: tt) => {
        path!("tests" / "inputs" / $str)
    };
}

#[test]
fn dies_when_input_file_not_exists() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg(input!("nonexistent.txt"))
        .assert()
        .failure()
        .stderr(str::contains("os error 2"));
    Ok(())
}

#[test_case("empty.txt", vec![], "empty.txt.out"; "empty")]
#[test_case("empty.txt", vec!["-c"], "empty.txt.c.out"; "empty with counts")]
#[test_case("one.txt", vec![], "one.txt.out"; "one")]
#[test_case("one.txt", vec!["-c"], "one.txt.c.out"; "one with counts")]
#[test_case("skip.txt", vec![], "skip.txt.out"; "skip")]
#[test_case("skip.txt", vec!["-c"], "skip.txt.c.out"; "skip with counts")]
#[test_case("t1.txt", vec![], "t1.txt.out"; "t1")]
#[test_case("t1.txt", vec!["-c"], "t1.txt.c.out"; "t1 with counts")]
#[test_case("t2.txt", vec![], "t2.txt.out"; "t2")]
#[test_case("t2.txt", vec!["-c"], "t2.txt.c.out"; "t2 with counts")]
#[test_case("t3.txt", vec![], "t3.txt.out"; "t3")]
#[test_case("t3.txt", vec!["-c"], "t3.txt.c.out"; "t3 with counts")]
#[test_case("t4.txt", vec![], "t4.txt.out"; "t4")]
#[test_case("t4.txt", vec!["-c"], "t4.txt.c.out"; "t4 with counts")]
#[test_case("t5.txt", vec![], "t5.txt.out"; "t5")]
#[test_case("t5.txt", vec!["-c"], "t5.txt.c.out"; "t5 with counts")]
#[test_case("t6.txt", vec![], "t6.txt.out"; "t6")]
#[test_case("t6.txt", vec!["-c"], "t6.txt.c.out"; "t6 with counts")]
#[test_case("three.txt", vec![], "three.txt.out"; "three")]
#[test_case("three.txt", vec!["-c"], "three.txt.c.out"; "three with counts")]
#[test_case("two.txt", vec![], "two.txt.out"; "two")]
#[test_case("two.txt", vec!["-c"], "two.txt.c.out"; "two with counts")]
fn run_in_file_out_file(file: &str, args: Vec<&str>, expected_file: &str) -> TestResult {
    let output_file = format!("{expected_file}.output");
    let expected_file = expected!(expected_file);
    let expected = fs::read(&expected_file)?;
    let expected = String::from_utf8_lossy(&expected).into_owned();

    let file = input!(file);
    Command::cargo_bin(PRG)?
        .arg(file)
        .arg(&output_file)
        .args(args)
        .assert()
        .success()
        .stdout(str::is_empty());
    let actual = fs::read(&output_file)?;
    let actual = String::from_utf8_lossy(&actual);
    assert_eq!(actual, expected);
    if std::path::Path::new(&output_file).exists() {
        fs::remove_file(&output_file)?;
    }
    Ok(())
}

#[test_case("empty.txt", vec![], "empty.txt.out"; "empty")]
#[test_case("empty.txt", vec!["-c"], "empty.txt.c.out"; "empty with counts")]
#[test_case("one.txt", vec![], "one.txt.out"; "one")]
#[test_case("one.txt", vec!["-c"], "one.txt.c.out"; "one with counts")]
#[test_case("skip.txt", vec![], "skip.txt.out"; "skip")]
#[test_case("skip.txt", vec!["-c"], "skip.txt.c.out"; "skip with counts")]
#[test_case("t1.txt", vec![], "t1.txt.out"; "t1")]
#[test_case("t1.txt", vec!["-c"], "t1.txt.c.out"; "t1 with counts")]
#[test_case("t2.txt", vec![], "t2.txt.out"; "t2")]
#[test_case("t2.txt", vec!["-c"], "t2.txt.c.out"; "t2 with counts")]
#[test_case("t3.txt", vec![], "t3.txt.out"; "t3")]
#[test_case("t3.txt", vec!["-c"], "t3.txt.c.out"; "t3 with counts")]
#[test_case("t4.txt", vec![], "t4.txt.out"; "t4")]
#[test_case("t4.txt", vec!["-c"], "t4.txt.c.out"; "t4 with counts")]
#[test_case("t5.txt", vec![], "t5.txt.out"; "t5")]
#[test_case("t5.txt", vec!["-c"], "t5.txt.c.out"; "t5 with counts")]
#[test_case("t6.txt", vec![], "t6.txt.out"; "t6")]
#[test_case("t6.txt", vec!["-c"], "t6.txt.c.out"; "t6 with counts")]
#[test_case("three.txt", vec![], "three.txt.out"; "three")]
#[test_case("three.txt", vec!["-c"], "three.txt.c.out"; "three with counts")]
#[test_case("two.txt", vec![], "two.txt.out"; "two")]
#[test_case("two.txt", vec!["-c"], "two.txt.c.out"; "two with counts")]
fn run_in_file_out_stdout(file: &str, args: Vec<&str>, expected_file: &str) -> TestResult {
    let expected_file = expected!(expected_file);
    let expected = fs::read(&expected_file)?;
    let expected = String::from_utf8_lossy(&expected).into_owned();

    let file = input!(file);
    Command::cargo_bin(PRG)?
        .arg(file)
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test_case("empty.txt", vec![], "empty.txt.out"; "empty")]
#[test_case("empty.txt", vec!["-c"], "empty.txt.c.out"; "empty with counts")]
#[test_case("one.txt", vec![], "one.txt.out"; "one")]
#[test_case("one.txt", vec!["-c"], "one.txt.c.out"; "one with counts")]
#[test_case("skip.txt", vec![], "skip.txt.out"; "skip")]
#[test_case("skip.txt", vec!["-c"], "skip.txt.c.out"; "skip with counts")]
#[test_case("t1.txt", vec![], "t1.txt.out"; "t1")]
#[test_case("t1.txt", vec!["-c"], "t1.txt.c.out"; "t1 with counts")]
#[test_case("t2.txt", vec![], "t2.txt.out"; "t2")]
#[test_case("t2.txt", vec!["-c"], "t2.txt.c.out"; "t2 with counts")]
#[test_case("t3.txt", vec![], "t3.txt.out"; "t3")]
#[test_case("t3.txt", vec!["-c"], "t3.txt.c.out"; "t3 with counts")]
#[test_case("t4.txt", vec![], "t4.txt.out"; "t4")]
#[test_case("t4.txt", vec!["-c"], "t4.txt.c.out"; "t4 with counts")]
#[test_case("t5.txt", vec![], "t5.txt.out"; "t5")]
#[test_case("t5.txt", vec!["-c"], "t5.txt.c.out"; "t5 with counts")]
#[test_case("t6.txt", vec![], "t6.txt.out"; "t6")]
#[test_case("t6.txt", vec!["-c"], "t6.txt.c.out"; "t6 with counts")]
#[test_case("three.txt", vec![], "three.txt.out"; "three")]
#[test_case("three.txt", vec!["-c"], "three.txt.c.out"; "three with counts")]
#[test_case("two.txt", vec![], "two.txt.out"; "two")]
#[test_case("two.txt", vec!["-c"], "two.txt.c.out"; "two with counts")]
fn run_in_stdin_out_file(file: &str, args: Vec<&str>, expected_file: &str) -> TestResult {
    let output_file = format!("{expected_file}.output");
    let expected_file = expected!(expected_file);
    let expected = fs::read(&expected_file)?;
    let expected = String::from_utf8_lossy(&expected).into_owned();

    let file = input!(file);
    Command::cargo_bin(PRG)?
        .arg("-")
        .arg(&output_file)
        .args(args)
        .pipe_stdin(file)?
        .assert()
        .success()
        .stdout(str::is_empty());
    let actual = fs::read(&output_file)?;
    let actual = String::from_utf8_lossy(&actual);
    assert_eq!(actual, expected);
    if std::path::Path::new(&output_file).exists() {
        fs::remove_file(&output_file)?;
    }
    Ok(())
}

#[test_case("empty.txt", vec![], "empty.txt.out"; "empty")]
#[test_case("empty.txt", vec!["-c"], "empty.txt.c.out"; "empty with counts")]
#[test_case("one.txt", vec![], "one.txt.out"; "one")]
#[test_case("one.txt", vec!["-c"], "one.txt.c.out"; "one with counts")]
#[test_case("skip.txt", vec![], "skip.txt.out"; "skip")]
#[test_case("skip.txt", vec!["-c"], "skip.txt.c.out"; "skip with counts")]
#[test_case("t1.txt", vec![], "t1.txt.out"; "t1")]
#[test_case("t1.txt", vec!["-c"], "t1.txt.c.out"; "t1 with counts")]
#[test_case("t2.txt", vec![], "t2.txt.out"; "t2")]
#[test_case("t2.txt", vec!["-c"], "t2.txt.c.out"; "t2 with counts")]
#[test_case("t3.txt", vec![], "t3.txt.out"; "t3")]
#[test_case("t3.txt", vec!["-c"], "t3.txt.c.out"; "t3 with counts")]
#[test_case("t4.txt", vec![], "t4.txt.out"; "t4")]
#[test_case("t4.txt", vec!["-c"], "t4.txt.c.out"; "t4 with counts")]
#[test_case("t5.txt", vec![], "t5.txt.out"; "t5")]
#[test_case("t5.txt", vec!["-c"], "t5.txt.c.out"; "t5 with counts")]
#[test_case("t6.txt", vec![], "t6.txt.out"; "t6")]
#[test_case("t6.txt", vec!["-c"], "t6.txt.c.out"; "t6 with counts")]
#[test_case("three.txt", vec![], "three.txt.out"; "three")]
#[test_case("three.txt", vec!["-c"], "three.txt.c.out"; "three with counts")]
#[test_case("two.txt", vec![], "two.txt.out"; "two")]
#[test_case("two.txt", vec!["-c"], "two.txt.c.out"; "two with counts")]
fn run_in_stdin_out_stdout(file: &str, args: Vec<&str>, expected_file: &str) -> TestResult {
    let expected_file = expected!(expected_file);
    let expected = fs::read(&expected_file)?;
    let expected = String::from_utf8_lossy(&expected).into_owned();

    let file = input!(file);
    Command::cargo_bin(PRG)?
        .arg("-")
        .args(args)
        .pipe_stdin(file)?
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}