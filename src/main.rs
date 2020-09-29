use clap::{App, AppSettings, ArgGroup};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let info = App::new("info")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::NoAutoVersion)
        .author("Nicolai Unrein <info@auxcontrol.io>")
        .about("Query package info from Cargo.toml in a script-friendly way.")
        .arg("-v --version      'get package version'")
        .arg("-a --authors      'get package authors'")
        .arg("-e --edition      'get package edition'")
        .arg("-n --name         'get package name'")
        .arg("-o --homepage     'get package homepage'")
        .arg("-k --keywords     'get package keywords'")
        .arg("-l --license      'get package license'")
        .arg("-i --links        'get package links'")
        .arg("-d --description  'get package description'")
        .arg("-c --categories   'get package categories'")
        .group(ArgGroup::new("info").required(true).args(&[
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
        ]));

    let app = App::new("Cargo Info").subcommand(info);
    let matches = app.get_matches();
    let matches = matches.subcommand_matches("info").unwrap();

    let p = search_manifest(&env::current_dir().unwrap()).unwrap();

    let manifest = cargo_toml::Manifest::from_path(p).unwrap();

    let package = manifest.package.unwrap();

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
    } else if matches.is_present("authors") {
        for line in package.authors {
            println!("{}", line);
        }
    } else if matches.is_present("links") {
        for line in package.links {
            println!("{}", line);
        }
    } else if matches.is_present("keywords") {
        for line in package.keywords {
            println!("{}", line);
        }
    } else if matches.is_present("categories") {
        for line in package.categories {
            println!("{}", line);
        }
    } else if matches.is_present("edition") {
        let edition = match package.edition {
            cargo_toml::Edition::E2015 => "2015",
            cargo_toml::Edition::E2018 => "2018",
        };
        println!("{}", edition);
    }
}

fn search_manifest(dir: &Path) -> Result<PathBuf> {
    let manifest = dir.join("Cargo.toml");

    if fs::metadata(&manifest).is_ok() {
        Ok(manifest)
    } else {
        search_manifest(dir.parent().unwrap())
    }
}
