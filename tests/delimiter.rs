use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_with_custom_delimiter() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("--authors").arg("--delimiter=;").assert();
    assert.success().stdout(predicate::eq(
        b"John Doe<john-doe@abc.com>;Jane Doe<jane-doe@def.com>\n" as &[u8],
    ));
}

#[test]
fn run_with_dashes_as_delimiters() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("--authors").arg("--delimiter= -- ").assert();
    assert.success().stdout(predicate::eq(
        b"John Doe<john-doe@abc.com> -- Jane Doe<jane-doe@def.com>\n" as &[u8],
    ));
}

#[test]
fn run_delimiters_no_equals() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("--delimiter").arg(" -- ").arg("--authors").assert();
    assert.success().stdout(predicate::eq(
        b"John Doe<john-doe@abc.com> -- Jane Doe<jane-doe@def.com>\n" as &[u8],
    ));
}

#[test]
fn run_delimiter_tab() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("--delimiter").arg("tab").arg("--authors").assert();
    assert.success().stdout(predicate::eq(
        b"John Doe<john-doe@abc.com>\tJane Doe<jane-doe@def.com>\n" as &[u8],
    ));
}
