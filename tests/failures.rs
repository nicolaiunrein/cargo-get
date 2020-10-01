use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_no_such_file_or_directory() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let assert = cmd.arg("--root=/tmp/this/does/not/exist").assert();
    assert.failure().stderr(predicate::eq(
        b"Error: \"No such file or directory\"\n" as &[u8],
    ));
}

#[test]
fn run_no_manifest_found() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let assert = cmd.arg("--root=/").assert();

    assert
        .failure()
        .stderr(predicate::eq(b"Error: \"No manifest found\"\n" as &[u8]));
}
