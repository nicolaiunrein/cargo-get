use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn workspace_members() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/workspace").unwrap();
    cmd.current_dir(p);

    cmd.arg("workspace.members")
        .assert()
        .success()
        .stdout(predicate::eq(b"pkg1\r\npkg2\n" as &[u8]));
}

#[test]
fn workspace_default_members() {
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/workspace").unwrap();
    cmd.current_dir(p);

    cmd.arg("workspace.default_members")
        .assert()
        .success()
        .stdout(predicate::eq(b"pkg2\n" as &[u8]));
}
