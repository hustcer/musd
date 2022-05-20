use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn runs_empty() {
    let mut cmd = Command::cargo_bin("musd").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE:\n"));
}

#[test]
fn runs_help() -> TestResult {
    Command::cargo_bin("musd")?
        .args(&["-h"])
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:\n"));
    Ok(())
}

#[test]
fn test_search() -> TestResult {
    let songs = musd::search("someone like you").unwrap();
    assert!(songs.len() > 5);
    Ok(())
}
