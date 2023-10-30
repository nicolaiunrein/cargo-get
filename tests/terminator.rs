use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn pkg_name_with_custom_terminator() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.name").arg("--terminator=.exe").assert();
    assert
        .success()
        .stdout(predicate::eq(b"test-name.exe" as &[u8]));
}

#[test]
fn pkg_name_with_cr_as_terminator() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.name").arg("--terminator=cr").assert();
    assert
        .success()
        .stdout(predicate::eq(b"test-name\r" as &[u8]));
}

#[test]
fn pkg_name_with_lf_as_terminator() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.name").arg("--terminator=lf").assert();
    assert
        .success()
        .stdout(predicate::eq(b"test-name\n" as &[u8]));
}

#[test]
fn pkg_name_with_crlf_as_terminator() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.name").arg("--terminator=crlf").assert();
    assert
        .success()
        .stdout(predicate::eq(b"test-name\r\n" as &[u8]));
}

#[test]
fn pkg_name_with_semicolon_as_terminator() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.name").arg("--terminator=;").assert();
    assert
        .success()
        .stdout(predicate::eq(b"test-name;" as &[u8]));
}

#[test]
fn pkg_name_with_nul_as_terminator() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.name").arg("--terminator=nul").assert();
    assert
        .success()
        .stdout(predicate::eq(b"test-name\0" as &[u8]));
}
