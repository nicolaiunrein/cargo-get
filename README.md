**Cargo plugin to easily query information from Cargo.toml files**

[![crates.io](https://img.shields.io/crates/v/cargo-get.svg)](https://crates.io/crates/cargo-get)
[![CI](https://github.com/nicolaiunrein/cargo-get/actions/workflows/ci.yaml/badge.svg)](https://github.com/nicolaiunrein/cargo-get/actions/workflows/ci.yaml)
[![build-binary](https://github.com/nicolaiunrein/cargo-get/actions/workflows/build-binary.yml/badge.svg)](https://github.com/nicolaiunrein/cargo-get/releases/latest)
[![github-actions](https://github.com/nicolaiunrein/cargo-get/actions/workflows/github-actions.yml/badge.svg)](https://github.com/nicolaiunrein/cargo-get/actions/workflows/github-actions.yml)

### Overview

This plugin helps querying information from a Cargo.toml file and can be used in shell scripts or CI/CD pipelines.
The plugin accepts only one flag at a time and returns a single string with the requested value.
Some queries like eq. keywords or authors return multiple values which get separated by line breaks (each value on a new line).

This plugin is inspired by [cargo-edit](https://github.com/killercup/cargo-edit) and presents itself as a lightweight wrapper around [cargo_toml](https://gitlab.com/crates.rs/cargo_toml)

### Contribution

Thanks for your interest - we gratefully welcome contributions.

Questions can be asked in issues.
To help us help you get pull requests merged quickly and smoothly, open an issue before submitted large changes. Please keep the contents of pull requests and commits short. Commit messages should include the intent of the commit.
cargo-get uses rustfmt for formatting and clippy for linting.

### Installation

### Pre-built Binaries

1. Download the binary for your CPU architecture from the [GitHub latest release][release].
2. Make the binary executable using `chmod +x`
3. Place the binary in your `$PATH` and rename it to `cargo-get`.


### Cargo

Ensure that you have a fairly recent version of rust/cargo installed.

```
$ cargo install cargo-get
```

(Please check cargo's documentation to learn how cargo install works and how to set up your system so it finds binaries installed by cargo.)

### Examples

#### All Options

```bash
$ cargo get -h
Cargo plugin to easily query information from Cargo.toml files

Usage: cargo-get [OPTIONS] <COMMAND>

Commands:
  package.name
  package.edition
  package.rust_version
  package.version
  package.build
  package.workspace
  package.authors
  package.links
  package.description
  package.homepage
  package.readme
  package.keywords
  package.categories
  package.exclude
  package.include
  package.license
  package.license_file
  package.repository
  package.default_run
  package.publish
  package.resolver
  package.metadata
  workspace.members
  workspace.package.authors
  workspace.package.categories
  workspace.package.description
  workspace.package.documentation
  workspace.package.edition
  workspace.package.exclude
  workspace.package.homepage
  workspace.package.include
  workspace.package.keywords
  workspace.package.license
  workspace.package.license_file
  workspace.package.publish
  workspace.package.readme
  workspace.package.repository
  workspace.package.rust_version
  workspace.package.version
  help                             Print this message or the help of the given subcommand(s)

Options:
      --entry <PATH>                               Path to the crate root to query
      --delimiter <Tab | CR | LF | CRLF | String>  Delimiter for array values
  -h, --help                                       Print help
  -V, --version                                    Print version
```

#### Get Version

```bash
$ cargo get package.version
0.2.1

$ cargo get package.version --pretty
v0.2.1

$ cargo get package.version --major --minor --patch --pre
0
2
1
alpha2

$ cargo get package.version --major --minor --delimiter="."
0.2

```

Now it is also easy to run commands like:

```bash
git tag $(cargo get package.version --pretty)
```

#### Get keywords

```bash
$ cargo get package.keywords
command-line-utilities
development-tools::cargo-plugins
```

#### Custom delimiter

```bash
# Use one of Tab, CR, LF, CRLF or a custom string.

$ cargo get package.keywords --delimiter Tab
command-line-utilities 	development-tools::cargo-plugins

$ cargo get package.keywords --delimiter=" -- "
command-line-utilities -- development-tools::cargo-plugins

$ cargo get package.keywords --delimiter=";"
command-line-utilities;development-tools::cargo-plugins
```

#### Optional entry point

```bash
# Full path
$ cargo get package.name --entry="../../some/other/project/Cargo.toml"
some-other-project

# Directory
$ cargo get package.name --entry="../../some/other/project"
some-other-project

# Current directory
$ cargo get package.name
current-project
```

### GitHub Actions

#### Package name

```yaml
      - name: Get package name
        id: cargo-get
        uses: nicolaiunrein/cargo-get@master
        with:
          subcommand: package.name
```

#### Package author

```yaml
      - name: Get package author
        id: cargo-get
        uses: nicolaiunrein/cargo-get@master
        with:
          subcommand: package.author
```

[release]: https://github.com/nicolaiunrein/cargo-get/releases/latest
