use assert_cmd::Command;
use path_macro::path;
use predicates::str;
use std::{fs, path::PathBuf};
use test_case::test_case;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "headr";

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
fn dies_when_not_number_lines() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(["-", "-n", "notnum"])
        .assert()
        .failure()
        .stderr(str::contains(
            "error: invalid value 'notnum' for '--lines <LINES>': invalid digit found in string",
        ));
    Ok(())
}

#[test]
fn dies_when_not_number_bytes() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(["-", "-c", "notnum"])
        .assert()
        .failure()
        .stderr(str::contains(
            "error: invalid value 'notnum' for '--bytes <BYTES>': invalid digit found in string",
        ));
    Ok(())
}

#[test]
fn dies_when_both_bytes_and_lines() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(["-", "-n", "10", "-c", "10"])
        .assert()
        .failure()
        .stderr(str::contains(
            "error: the argument '--lines <LINES>' cannot be used with '--bytes <BYTES>'",
        ));
    Ok(())
}

#[test_case(vec!["empty.txt"], vec![], "empty.txt.out"; "empty")]
#[test_case(vec!["empty.txt"], vec!["-c", "1"], "empty.txt.c1.out"; "empty bytes 1")]
#[test_case(vec!["empty.txt"], vec!["-c", "2"], "empty.txt.c2.out"; "empty bytes 2")]
#[test_case(vec!["empty.txt"], vec!["-c", "4"], "empty.txt.c4.out"; "empty bytes 4")]
#[test_case(vec!["empty.txt"], vec!["-n", "2"], "empty.txt.n2.out"; "empty lines 2")]
#[test_case(vec!["empty.txt"], vec!["-n", "4"], "empty.txt.n4.out"; "empty lines 4")]
#[test_case(vec!["one.txt"], vec![], "one.txt.out"; "one")]
#[test_case(vec!["one.txt"], vec!["-c", "1"], "one.txt.c1.out"; "one bytes 1")]
#[test_case(vec!["one.txt"], vec!["-c", "2"], "one.txt.c2.out"; "one bytes 2")]
#[test_case(vec!["one.txt"], vec!["-c", "4"], "one.txt.c4.out"; "one bytes 4")]
#[test_case(vec!["one.txt"], vec!["-n", "2"], "one.txt.n2.out"; "one lines 2")]
#[test_case(vec!["one.txt"], vec!["-n", "4"], "one.txt.n4.out"; "one lines 4")]
#[test_case(vec!["two.txt"], vec![], "two.txt.out"; "two")]
#[test_case(vec!["two.txt"], vec!["-c", "1"], "two.txt.c1.out"; "two bytes 1")]
#[test_case(vec!["two.txt"], vec!["-c", "2"], "two.txt.c2.out"; "two bytes 2")]
#[test_case(vec!["two.txt"], vec!["-c", "4"], "two.txt.c4.out"; "two bytes 4")]
#[test_case(vec!["two.txt"], vec!["-n", "2"], "two.txt.n2.out"; "two lines 2")]
#[test_case(vec!["two.txt"], vec!["-n", "4"], "two.txt.n4.out"; "two lines 4")]
#[test_case(vec!["three.txt"], vec![], "three.txt.out"; "three")]
#[test_case(vec!["three.txt"], vec!["-c", "1"], "three.txt.c1.out"; "three bytes 1")]
#[test_case(vec!["three.txt"], vec!["-c", "2"], "three.txt.c2.out"; "three bytes 2")]
#[test_case(vec!["three.txt"], vec!["-c", "4"], "three.txt.c4.out"; "three bytes 4")]
#[test_case(vec!["three.txt"], vec!["-n", "2"], "three.txt.n2.out"; "three lines 2")]
#[test_case(vec!["three.txt"], vec!["-n", "4"], "three.txt.n4.out"; "three lines 4")]
#[test_case(vec!["ten.txt"], vec![], "ten.txt.out"; "ten")]
#[test_case(vec!["ten.txt"], vec!["-c", "1"], "ten.txt.c1.out"; "ten bytes 1")]
#[test_case(vec!["ten.txt"], vec!["-c", "2"], "ten.txt.c2.out"; "ten bytes 2")]
#[test_case(vec!["ten.txt"], vec!["-c", "4"], "ten.txt.c4.out"; "ten bytes 4")]
#[test_case(vec!["ten.txt"], vec!["-n", "2"], "ten.txt.n2.out"; "ten lines 2")]
#[test_case(vec!["ten.txt"], vec!["-n", "4"], "ten.txt.n4.out"; "ten lines 4")]
#[test_case(vec!["empty.txt", "one.txt", "two.txt", "three.txt", "ten.txt"], vec![], "all.out"; "all")]
#[test_case(vec!["empty.txt", "one.txt", "two.txt", "three.txt", "ten.txt"], vec!["-c", "1"], "all.c1.out"; "all bytes 1")]
#[test_case(vec!["empty.txt", "one.txt", "two.txt", "three.txt", "ten.txt"], vec!["-c", "2"], "all.c2.out"; "all bytes 2")]
#[test_case(vec!["empty.txt", "one.txt", "two.txt", "three.txt", "ten.txt"], vec!["-c", "4"], "all.c4.out"; "all bytes 4")]
#[test_case(vec!["empty.txt", "one.txt", "two.txt", "three.txt", "ten.txt"], vec!["-n", "2"], "all.n2.out"; "all lines 2")]
#[test_case(vec!["empty.txt", "one.txt", "two.txt", "three.txt", "ten.txt"], vec!["-n", "4"], "all.n4.out"; "all lines 4")]
fn run(files: Vec<&str>, args: Vec<&str>, expected_file: &str) -> TestResult {
    let expected_file = expected!(expected_file);
    let expected = fs::read(&expected_file)?;
    let expected = String::from_utf8_lossy(&expected).into_owned();

    let files: Vec<PathBuf> = files.iter().map(|x| input!(x)).collect();
    Command::cargo_bin(PRG)?
        .args(files)
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test_case("empty.txt", vec![], "empty.txt.out"; "empty")]
#[test_case("empty.txt", vec!["-c", "1"], "empty.txt.c1.out"; "empty bytes 1")]
#[test_case("empty.txt", vec!["-c", "2"], "empty.txt.c2.out"; "empty bytes 2")]
#[test_case("empty.txt", vec!["-c", "4"], "empty.txt.c4.out"; "empty bytes 4")]
#[test_case("empty.txt", vec!["-n", "2"], "empty.txt.n2.out"; "empty lines 2")]
#[test_case("empty.txt", vec!["-n", "4"], "empty.txt.n4.out"; "empty lines 4")]
#[test_case("one.txt", vec![], "one.txt.out"; "one")]
#[test_case("one.txt", vec!["-c", "1"], "one.txt.c1.out"; "one bytes 1")]
#[test_case("one.txt", vec!["-c", "2"], "one.txt.c2.out"; "one bytes 2")]
#[test_case("one.txt", vec!["-c", "4"], "one.txt.c4.out"; "one bytes 4")]
#[test_case("one.txt", vec!["-n", "2"], "one.txt.n2.out"; "one lines 2")]
#[test_case("one.txt", vec!["-n", "4"], "one.txt.n4.out"; "one lines 4")]
#[test_case("two.txt", vec![], "two.txt.out"; "two")]
#[test_case("two.txt", vec!["-c", "1"], "two.txt.c1.out"; "two bytes 1")]
#[test_case("two.txt", vec!["-c", "2"], "two.txt.c2.out"; "two bytes 2")]
#[test_case("two.txt", vec!["-c", "4"], "two.txt.c4.out"; "two bytes 4")]
#[test_case("two.txt", vec!["-n", "2"], "two.txt.n2.out"; "two lines 2")]
#[test_case("two.txt", vec!["-n", "4"], "two.txt.n4.out"; "two lines 4")]
#[test_case("three.txt", vec![], "three.txt.out"; "three")]
#[test_case("three.txt", vec!["-c", "1"], "three.txt.c1.out"; "three bytes 1")]
#[test_case("three.txt", vec!["-c", "2"], "three.txt.c2.out"; "three bytes 2")]
#[test_case("three.txt", vec!["-c", "4"], "three.txt.c4.out"; "three bytes 4")]
#[test_case("three.txt", vec!["-n", "2"], "three.txt.n2.out"; "three lines 2")]
#[test_case("three.txt", vec!["-n", "4"], "three.txt.n4.out"; "three lines 4")]
#[test_case("ten.txt", vec![], "ten.txt.out"; "ten")]
#[test_case("ten.txt", vec!["-c", "1"], "ten.txt.c1.out"; "ten bytes 1")]
#[test_case("ten.txt", vec!["-c", "2"], "ten.txt.c2.out"; "ten bytes 2")]
#[test_case("ten.txt", vec!["-c", "4"], "ten.txt.c4.out"; "ten bytes 4")]
#[test_case("ten.txt", vec!["-n", "2"], "ten.txt.n2.out"; "ten lines 2")]
#[test_case("ten.txt", vec!["-n", "4"], "ten.txt.n4.out"; "ten lines 4")]
fn run_stdio(file: &str, args: Vec<&str>, expected_file: &str) -> TestResult {
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
