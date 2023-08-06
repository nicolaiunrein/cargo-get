use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_version_full() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_04").unwrap();
    cmd.current_dir(p);

    cmd.arg("version")
        .arg("--full")
        .assert()
        .success()
        .stdout(predicate::eq(b"1.2.3\n" as &[u8]));
}
