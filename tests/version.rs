use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_version_without_args_fails() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let assert = cmd.assert();
    assert.failure();
}

#[test]
fn run_version_full() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.version")
        .arg("--full")
        .assert()
        .success()
        .stdout(predicate::eq(b"1.2.3\n" as &[u8]));
}

#[test]
fn run_version_pretty() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.version")
        .arg("--pretty")
        .assert()
        .success()
        .stdout(predicate::eq(b"v1.2.3\n" as &[u8]));
}

#[test]
fn run_version_major() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.version")
        .arg("--major")
        .assert()
        .success()
        .stdout(predicate::eq(b"1\n" as &[u8]));
}

#[test]
fn run_version_minor() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.version")
        .arg("--minor")
        .assert()
        .success()
        .stdout(predicate::eq(b"2\n" as &[u8]));
}

#[test]
fn run_version_patch() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.version")
        .arg("--patch")
        .assert()
        .success()
        .stdout(predicate::eq(b"3\n" as &[u8]));
}

#[test]
fn run_version_build() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.version")
        .arg("--build")
        .assert()
        .success()
        .stdout(predicate::eq(b"build-2\n" as &[u8]));
}

#[test]
fn run_version_pre() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_03").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.version")
        .arg("--pre")
        .assert()
        .success()
        .stdout(predicate::eq(b"alpha-3\n" as &[u8]));
}
