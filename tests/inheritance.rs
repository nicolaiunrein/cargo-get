use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn inherited_version() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/workspace/pkg1").unwrap();
    cmd.current_dir(p);

    cmd.arg("package.version")
        .assert()
        .success()
        .stdout(predicate::eq(b"1.2.3\n" as &[u8]));
}

#[test]
fn workspace_version() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/workspace").unwrap();
    cmd.current_dir(p);

    cmd.arg("workspace.package.version")
        .arg("--pretty")
        .assert()
        .success()
        .stdout(predicate::eq(b"v1.2.3\n" as &[u8]));
}
