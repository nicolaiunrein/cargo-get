use assert_cmd::Command;

const EXPECTED: &str = r"package_name<<EOF
test-name
EOF
package_edition<<EOF
2018
EOF
package_version<<EOF
1.2.3
EOF
package_authors<<EOF
John Doe<john-doe@abc.com>
EOF
package_description<<EOF
A very useful description
EOF
package_readme<<EOF
README.md
EOF
package_keywords<<EOF
binary,cargo,cli,dev-tools,query
EOF
package_categories<<EOF

EOF
package_exclude<<EOF

EOF
package_include<<EOF

EOF
package_license<<EOF
Apache-2.0/MIT
EOF
package_repository<<EOF
https://github.com/nicolaiunrein/cargo-get
EOF
package_publish<<EOF
true
EOF
";

#[test]
fn run_multi() {
    use pretty_assertions::assert_eq;
    let mut cmd = Command::cargo_bin("cargo-get").unwrap();
    let p = std::fs::canonicalize("tests/data/toml_01").unwrap();
    cmd.current_dir(p);
    let assert = cmd
        .arg("all")
        .arg("--delimiter")
        .arg(",")
        .arg("--output-format")
        .arg("github-action")
        .assert();

    let res = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(res, EXPECTED);
}
