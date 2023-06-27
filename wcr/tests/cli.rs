use assert_cmd::Command;
use path_macro::path;
use predicates::str;
use std::{fs, path::PathBuf};
use test_case::test_case;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "wcr";

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
fn dies_when_both_bytes_and_words() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(["-", "-cm"])
        .assert()
        .failure()
        .stderr(str::contains(
            "error: the argument '--bytes' cannot be used with '--chars'",
        ));
    Ok(())
}

#[test_case(vec!["empty.txt"], vec![], "empty.txt.out"; "empty")]
#[test_case(vec!["empty.txt"], vec!["-c"], "empty.txt.c.out"; "empty bytes")]
#[test_case(vec!["empty.txt"], vec!["-cl"], "empty.txt.cl.out"; "empty lines bytes")]
#[test_case(vec!["empty.txt"], vec!["-l"], "empty.txt.l.out"; "empty lines")]
#[test_case(vec!["empty.txt"], vec!["-lwm"], "empty.txt.lwm.out"; "empty lines words chars")]
#[test_case(vec!["empty.txt"], vec!["-m"], "empty.txt.m.out"; "empty chars")]
#[test_case(vec!["empty.txt"], vec!["-ml"], "empty.txt.ml.out"; "empty lines chars")]
#[test_case(vec!["empty.txt"], vec!["-w"], "empty.txt.w.out"; "empty words")]
#[test_case(vec!["empty.txt"], vec!["-wc"], "empty.txt.wc.out"; "empty words bytes")]
#[test_case(vec!["empty.txt"], vec!["-wl"], "empty.txt.wl.out"; "empty lines words")]
#[test_case(vec!["empty.txt"], vec!["-wm"], "empty.txt.wm.out"; "empty words chars")]
#[test_case(vec!["fox.txt"], vec![], "fox.txt.out"; "fox")]
#[test_case(vec!["fox.txt"], vec!["-c"], "fox.txt.c.out"; "fox bytes")]
#[test_case(vec!["fox.txt"], vec!["-cl"], "fox.txt.cl.out"; "fox lines bytes")]
#[test_case(vec!["fox.txt"], vec!["-l"], "fox.txt.l.out"; "fox lines")]
#[test_case(vec!["fox.txt"], vec!["-lwm"], "fox.txt.lwm.out"; "fox lines words chars")]
#[test_case(vec!["fox.txt"], vec!["-m"], "fox.txt.m.out"; "fox chars")]
#[test_case(vec!["fox.txt"], vec!["-ml"], "fox.txt.ml.out"; "fox lines chars")]
#[test_case(vec!["fox.txt"], vec!["-w"], "fox.txt.w.out"; "fox words")]
#[test_case(vec!["fox.txt"], vec!["-wc"], "fox.txt.wc.out"; "fox words bytes")]
#[test_case(vec!["fox.txt"], vec!["-wl"], "fox.txt.wl.out"; "fox lines words")]
#[test_case(vec!["fox.txt"], vec!["-wm"], "fox.txt.wm.out"; "fox words chars")]
#[test_case(vec!["atlamal.txt"], vec![], "atlamal.txt.out"; "atlamal")]
#[test_case(vec!["atlamal.txt"], vec!["-c"], "atlamal.txt.c.out"; "atlamal bytes")]
#[test_case(vec!["atlamal.txt"], vec!["-cl"], "atlamal.txt.cl.out"; "atlamal lines bytes")]
#[test_case(vec!["atlamal.txt"], vec!["-l"], "atlamal.txt.l.out"; "atlamal lines")]
#[test_case(vec!["atlamal.txt"], vec!["-lwm"], "atlamal.txt.lwm.out"; "atlamal lines words chars")]
#[test_case(vec!["atlamal.txt"], vec!["-m"], "atlamal.txt.m.out"; "atlamal chars")]
#[test_case(vec!["atlamal.txt"], vec!["-ml"], "atlamal.txt.ml.out"; "atlamal lines chars")]
#[test_case(vec!["atlamal.txt"], vec!["-w"], "atlamal.txt.w.out"; "atlamal words")]
#[test_case(vec!["atlamal.txt"], vec!["-wc"], "atlamal.txt.wc.out"; "atlamal words bytes")]
#[test_case(vec!["atlamal.txt"], vec!["-wl"], "atlamal.txt.wl.out"; "atlamal lines words")]
#[test_case(vec!["atlamal.txt"], vec!["-wm"], "atlamal.txt.wm.out"; "atlamal words chars")]
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

#[test_case("atlamal.txt", "atlamal.txt.stdin.out"; "atlamal stdin")]
fn run_stdin(file: &str, expected_file: &str) -> TestResult {
    let expected_file = expected!(expected_file);
    let expected = fs::read(&expected_file)?;
    let expected = String::from_utf8_lossy(&expected).into_owned();

    let file = input!(file);
    Command::cargo_bin(PRG)?
        .pipe_stdin(file)?
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
