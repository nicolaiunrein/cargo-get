use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_no_such_file_or_directory() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let assert = cmd
        .arg("--entry=/tmp/this/does/not/exist")
        .arg("package.name")
        .assert();
    assert.failure().stderr(predicate::eq(
        b"Error: Missing/invalid entry point [/tmp/this/does/not/exist]\n" as &[u8],
    ));
}

#[test]
fn run_no_manifest_found() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let assert = cmd.arg("--entry=/").arg("package.name").assert();

    assert
        .failure()
        .stderr(predicate::eq(b"Error: No manifest found\n" as &[u8]));
}

#[test]
fn run_version_pretty_minor_conflict() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("version")
        .arg("--pretty")
        .arg("--minor")
        .assert()
        .failure();
}
#[test]
fn run_version_pretty_full_conflict() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("version")
        .arg("--pretty")
        .arg("--full")
        .assert()
        .failure();
}
#[test]
fn run_version_full_minor_conflict() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("version")
        .arg("--full")
        .arg("--minor")
        .assert()
        .failure();
}
