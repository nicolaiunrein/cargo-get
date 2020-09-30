mod delimiter;

use cargo_toml::Manifest;
use clap::{App, AppSettings, ArgGroup, ArgMatches};
use delimiter::Delimiter;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    let app = make_app();

    let matches = app.get_matches_from(args);

    let entry_point = match matches.value_of("root") {
        Some(p) => p.parse()?,
        None => env::current_dir()?,
    };

    let entry_point_absolute =
        fs::canonicalize(&entry_point).map_err(|_| "No such file or directory")?;

    let manifest_path =
        search_manifest_path(&entry_point_absolute).ok_or_else(|| r#"No manifest found"#)?;

    let manifest = Manifest::from_path(manifest_path)?;

    output(&matches, manifest)
}

// Remove get argument in order to make it work with or without `get` subcommand
fn get_args() -> Vec<String> {
    let mut args: Vec<_> = std::env::args().collect();

    if args.get(1) == Some(&"get".to_owned()) {
        args.remove(1);
    }

    args
}

pub fn make_app() -> App<'static> {
    App::new("cargo-get")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::NoAutoVersion)
        .author("Nicolai Unrein <info@auxcontrol.io>")
        .about("Query package info from Cargo.toml in a script-friendly way.")
        .arg("-v --version                                      'get package version'")
        .arg("-a --authors                                      'get package authors'")
        .arg("-e --edition                                      'get package edition'")
        .arg("-n --name                                         'get package name'")
        .arg("-o --homepage                                     'get package homepage'")
        .arg("-k --keywords                                     'get package keywords'")
        .arg("-l --license                                      'get package license'")
        .arg("-i --links                                        'get package links'")
        .arg("-d --description                                  'get package description'")
        .arg("-c --categories                                   'get package categories'")
        .arg("--root [Path]                                     'optional entry point'")
        .arg("--delimiter [Tab | CR | LF | CRLF | String]       'specify delimiter for values'")
        .group(ArgGroup::new("get").required(true).args(&[
            "version",
            "authors",
            "edition",
            "name",
            "homepage",
            "keywords",
            "license",
            "links",
            "description",
            "categories",
        ]))
}

pub fn output(matches: &ArgMatches, manifest: Manifest) -> Result<(), Box<dyn Error>> {
    let package = manifest.package.ok_or_else(|| "Package not found")?;

    let delimiter: Delimiter = matches
        .value_of("delimiter")
        .map(|s| s.parse().unwrap())
        .unwrap_or_default();

    let delim_string = delimiter.to_string();

    if matches.is_present("version") {
        println!("{}", package.version);
    } else if matches.is_present("name") {
        println!("{}", package.name);
    } else if matches.is_present("homepage") {
        println!("{}", package.homepage.unwrap_or_default());
    } else if matches.is_present("license") {
        println!("{}", package.license.unwrap_or_default());
    } else if matches.is_present("description") {
        println!("{}", package.description.unwrap_or_default());
    } else if matches.is_present("links") {
        println!("{}", package.links.unwrap_or_default());
    } else if matches.is_present("authors") {
        println!("{}", package.authors.join(&delim_string))
    } else if matches.is_present("keywords") {
        println!("{}", package.keywords.join(&delim_string))
    } else if matches.is_present("categories") {
        println!("{}", package.categories.join(&delim_string))
    } else if matches.is_present("edition") {
        let edition = match package.edition {
            cargo_toml::Edition::E2015 => "2015",
            cargo_toml::Edition::E2018 => "2018",
        };
        println!("{}", edition);
    }
    Ok(())
}

fn search_manifest_path(dir: &Path) -> Option<PathBuf> {
    let manifest = dir.join("Cargo.toml");

    if fs::metadata(&manifest).is_ok() {
        Some(manifest)
    } else {
        dir.parent().map(search_manifest_path).flatten()
    }
}
