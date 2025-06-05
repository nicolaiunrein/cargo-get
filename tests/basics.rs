use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_without_args_fails() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let assert = cmd.assert();
    assert.failure();
}

#[test]
fn run_name() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.name").assert();
    assert
        .success()
        .stdout(predicate::eq(b"test-name\n" as &[u8]));
}

#[test]
fn run_homepage_empty() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.homepage").assert().failure();
}

#[test]
fn run_homepage_present() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.homepage").assert();
    assert
        .success()
        .stdout(predicate::eq(b"crates.io\n" as &[u8]));
}

#[test]
fn run_author() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.authors").assert();
    assert
        .success()
        .stdout(predicate::eq(b"John Doe<john-doe@abc.com>\n" as &[u8]));
}

#[test]
fn run_multiple_author() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.authors").assert();
    assert.success().stdout(predicate::eq(
        b"John Doe<john-doe@abc.com>\r\nJane Doe<jane-doe@def.com>\n" as &[u8],
    ));
}

#[test]
fn run_keywords() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.keywords").assert();
    assert.success().stdout(predicate::eq(
        b"binary\r\ncargo\r\ncli\r\ndev-tools\r\nquery\n" as &[u8],
    ));
}

#[test]
fn run_license() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.license").assert();
    assert
        .success()
        .stdout(predicate::eq(b"Apache-2.0/MIT\n" as &[u8]));
}

#[test]
fn run_links_empty() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.links").assert().failure();
}

#[test]
fn run_links() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.links").assert();
    assert.success().stdout(predicate::eq(b"foo\n" as &[u8]));
}

#[test]
fn run_description() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.description").assert();
    assert
        .success()
        .stdout(predicate::eq(b"A very useful description\n" as &[u8]));
}

#[test]
fn run_description_missing() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_06").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.description").assert();
    assert.failure().stderr(predicate::eq(
        b"Error: `package.description` not specified in manifest\n" as &[u8],
    ));
}

#[test]
fn run_categories_empty() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.categories").assert();
    assert.success().stdout(predicate::eq(b"\n" as &[u8]));
}

#[test]
fn run_categories_one() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.categories").assert();
    assert.success().stdout(predicate::eq(b"cli\n" as &[u8]));
}

#[test]
fn run_categories_multiple() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_03").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.categories").assert();
    assert
        .success()
        .stdout(predicate::eq(b"cli\r\nconfig\n" as &[u8]));
}

#[test]
fn run_edition_2021() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_03").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.edition").assert();
    assert.success().stdout(predicate::eq(b"2021\n" as &[u8]));
}

#[test]
fn run_edition_2018() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.edition").assert();
    assert.success().stdout(predicate::eq(b"2018\n" as &[u8]));
}

#[test]
fn run_edition_2015() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_02").unwrap();
    cmd.current_dir(p);

    let assert = cmd.arg("package.edition").assert();
    assert.success().stdout(predicate::eq(b"2015\n" as &[u8]));
}
