**Cargo plugin to easily query information from Cargo.toml file.**

### Overview
This plugin helps querying information from a Cargo.toml file and can be used in shell scripts or CI/CD pipelines.
The plugin accepts only one flag at a time and returns a single string with the requested value.
Some queries like eq. keywords or authors return multiple values which get separated by line breaks (each value on a new line).

### Basic Example

#### All Options
```
$ cargo info -h
Cargo Info
Nicolai Unrein <info@auxcontrol.io>
Query package info from Cargo.toml in a script-friendly way.

USAGE:
    cargo_info <--version|--authors|--edition|--name|--homepage|--keywords|--license|--links|--description|--categories>

FLAGS:
    -v, --version        get package version
    -a, --authors        get package authors
    -e, --edition        get package edition
    -n, --name           get package name
    -o, --homepage       get package homepage
    -k, --keywords       get package keywords
    -l, --license        get package license
    -i, --links          get package links
    -d, --description    get package description
    -c, --categories     get package categories
    -h, --help           Prints help information
```

#### Get Version
```
$ cargo info -v
0.2.1

```

#### Get keywords 
```
$ cargo info -k
command-line-utilities
development-tools::cargo-plugins

```
