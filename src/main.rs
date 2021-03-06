mod delimiter;

use cargo_toml::Manifest;
use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches};
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
        search_manifest_path(&entry_point_absolute).ok_or(r#"No manifest found"#)?;

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
        .author("Nicolai Unrein <n.unrein@gmail.com>")
        .about("Query package info from Cargo.toml in a script-friendly way.")
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
        .arg(
            Arg::from(
                "--delimiter [Tab | CR | LF | CRLF | String]       'specify delimiter for values'",
            )
            .global(true),
        )
        .group(ArgGroup::new("version-group").requires("version"))
        .group(ArgGroup::new("get").required(false).args(&[
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
        .subcommand(
            App::new("version")
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::GlobalVersion)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::NoAutoVersion)
                .about("get package version")
                .arg(
                    Arg::from("--full 'get full version'")
                        .conflicts_with_all(&["major", "minor", "patch", "build", "pre", "pretty"])
                        .hidden(true),
                )
                .arg(
                    Arg::from("--pretty 'get pretty version eg. v1.2.3'")
                        .conflicts_with_all(&["major", "minor", "patch", "build", "pre", "full"]),
                )
                .arg("--major                                   'get major part'")
                .arg("--minor                                   'get minor part'")
                .arg("--patch                                   'get patch part'")
                .arg("--build                                   'get build part'")
                .arg("--pre                                     'get pre-release part'"),
        )
}

pub fn output(matches: &ArgMatches, manifest: Manifest) -> Result<(), Box<dyn Error>> {
    let package = manifest.package.ok_or("Package not found")?;

    let delimiter: Delimiter = matches
        .value_of("delimiter")
        .map(|s| s.parse().unwrap())
        .unwrap_or_default();

    let delim_string = delimiter.to_string();

    if let Some(version) = matches.subcommand_matches("version") {
        let mut out = Vec::new();
        let v: semver::Version = package.version.parse().unwrap();

        if version.is_present("full") {
            println!("{}", v);
            return Ok(());
        }

        if version.is_present("pretty") {
            println!("v{}", v);
            return Ok(());
        }

        if version.is_present("major") {
            out.push(v.major.to_string());
        }

        if version.is_present("minor") {
            out.push(v.minor.to_string());
        }

        if version.is_present("patch") {
            out.push(v.patch.to_string())
        }

        if version.is_present("build") {
            for b in v.build.iter() {
                out.push(format!("{}", b))
            }
        }
        if version.is_present("pre") {
            for p in v.pre.iter() {
                out.push(format!("{}", p))
            }
        }
        if out.is_empty() {
            out.push(format!("{}", v));
        }
        println!("{}", out.join(&delim_string));
        return Ok(());
    }

    if matches.is_present("name") {
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
