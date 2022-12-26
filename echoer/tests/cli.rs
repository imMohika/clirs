use assert_cmd::Command;
use predicates::prelude::predicate;
use std::error;

type TestResult = Result<(), Box<dyn error::Error>>;

#[test]
fn dies_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echoer")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echoer")?;
    cmd.arg("num").assert().success();
    Ok(())
}

#[test]
fn num() -> TestResult {
    let expected = "num num\n";
    let mut cmd = Command::cargo_bin("echoer")?;
    cmd.arg("num num").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn num_arr() -> TestResult {
    let expected = "num num\n";
    let mut cmd = Command::cargo_bin("echoer")?;
    cmd.args(&["num", "num"])
        .assert()
        .success()
        .stdout(expected.clone());
    Ok(())
}

#[test]
fn num_n() -> TestResult {
    let expected = "num num";
    let mut cmd = Command::cargo_bin("echoer")?;
    cmd.args(["num num", "-n"])
        .assert()
        .success()
        .stdout(expected.clone());
    Ok(())
}

#[test]
fn num_n_arr() -> TestResult {
    let expected = "num num";
    let mut cmd = Command::cargo_bin("echoer")?;
    cmd.args(&["-n", "num", "num"])
        .assert()
        .success()
        .stdout(expected.clone());
    Ok(())
}
