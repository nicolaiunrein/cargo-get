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
        fs::canonicalize(entry_point).map_err(|_| "No such file or directory")?;

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

pub fn make_app() -> App<'static, 'static> {
    App::new("cargo-get")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .author("Nicolai Unrein <n.unrein@gmail.com>")
        .about("Query package info from Cargo.toml in a script-friendly way.")
        .arg(
            Arg::with_name("authors")
                .long("authors")
                .short("a")
                .help("get package authors"),
        )
        .arg(
            Arg::with_name("edition")
                .long("edition")
                .short("e")
                .help("get package edition"),
        )
        .arg(
            Arg::with_name("name")
                .long("name")
                .short("n")
                .help("get package name"),
        )
        .arg(
            Arg::with_name("homepage")
                .long("homepage")
                .short("o")
                .help("get package homepage"),
        )
        .arg(
            Arg::with_name("keywords")
                .long("keywords")
                .short("k")
                .help("get package keywords"),
        )
        .arg(
            Arg::with_name("license")
                .long("license")
                .short("l")
                .help("get package license"),
        )
        .arg(
            Arg::with_name("links")
                .long("links")
                .short("i")
                .help("get package links"),
        )
        .arg(
            Arg::with_name("description")
                .long("description")
                .short("d")
                .help("get package description"),
        )
        .arg(
            Arg::with_name("categories")
                .long("categories")
                .short("c")
                .help("get package categories"),
        )
        .arg(
            Arg::with_name("root")
                .long("root")
                .help("optional entry point")
                .value_name("PATH"),
        )
        .arg(
            Arg::with_name("delimiter")
                .long("delimiter")
                .help("specify delimiter for values")
                .value_name("Tab | CR | LF | CRLF | String")
                .global(true),
        )
        .group(ArgGroup::with_name("version-group").requires("version"))
        .group(ArgGroup::with_name("get").required(false).args(&[
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
                .setting(AppSettings::VersionlessSubcommands)
                .about("get package version")
                .arg(
                    Arg::with_name("full")
                        .long("full")
                        .help("get full version")
                        .conflicts_with_all(&["major", "minor", "patch", "build", "pre", "pretty"])
                        .hidden(true),
                )
                .arg(
                    Arg::with_name("pretty")
                        .long("pretty")
                        .help("get pretty version eg. v1.2.3")
                        .conflicts_with_all(&["major", "minor", "patch", "build", "pre", "full"]),
                )
                .arg(Arg::with_name("major").long("major").help("get major part"))
                .arg(Arg::with_name("minor").long("minor").help("get minor part"))
                .arg(Arg::with_name("patch").long("patch").help("get patch part"))
                .arg(Arg::with_name("build").long("build").help("get build part"))
                .arg(
                    Arg::with_name("pre")
                        .long("pre")
                        .help("get pre-release part"),
                ),
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
            cargo_toml::Edition::E2021 => "2021",
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
        dir.parent().and_then(search_manifest_path)
    }
}
