mod cli;
mod delimiter;
mod error;

use cargo_toml::Manifest;
use clap::Parser;
use delimiter::Delimiter;
use error::{InheritanceError, NotFound};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse_from(get_args());
    let entry_point = match cli.root {
        Some(p) => p,
        None => std::env::current_dir()?,
    };

    let entry_point_absolute =
        std::fs::canonicalize(entry_point).map_err(|_| "No such file or directory")?;

    let manifest_path = search_manifest_path(&entry_point_absolute).ok_or("No manifest found")?;

    let manifest = Manifest::from_path(&manifest_path)?;

    let package = || manifest.package.clone().ok_or(NotFound("package"));
    let workspace = || manifest.workspace.clone().ok_or(NotFound("workspace"));
    let ws_package = || workspace().and_then(|ws| ws.package.ok_or(NotFound("workspace.package")));

    let delimiter: Delimiter = cli.delimiter.unwrap_or_default();
    let delim_string = delimiter.to_string();

    let output = match cli.command {
        cli::Command::PackageVersion { inner } => {
            let v: semver::Version = package()?.version().parse()?;
            inner.match_version(v, &delimiter)?
        }
        cli::Command::PackageAuthors => package()?
            .authors
            .get()
            .or(Err(InheritanceError("package.authors")))?
            .join(&delim_string),

        cli::Command::PackageEdition => match package()?.edition() {
            cargo_toml::Edition::E2015 => "2015",
            cargo_toml::Edition::E2018 => "2018",
            cargo_toml::Edition::E2021 => "2021",
        }
        .to_string(),
        cli::Command::PackageName => package()?.name().to_string(),
        cli::Command::PackageHomepage => package()?.homepage().unwrap_or_default().to_string(),
        cli::Command::PackageKeywords => package()?.keywords().join(&delim_string),
        cli::Command::PackageLicense => package()?.license().unwrap_or_default().to_string(),
        cli::Command::PackageLinks => package()?.links().unwrap_or_default().to_string(),
        cli::Command::PackageDescription => {
            package()?.description().unwrap_or_default().to_string()
        }
        cli::Command::PackageCategories => package()?.categories().join(&delim_string),
        cli::Command::WorkspaceMembers => workspace()?.members.join(&delim_string),
        cli::Command::WorkspacePackageVersion { inner } => {
            let v: semver::Version = package()?.version().parse()?;
            inner.match_version(v, &delimiter)?
        }
        cli::Command::WorkspacePackageAuthors => ws_package()?
            .authors
            .unwrap_or_default()
            .join(&delim_string),
        cli::Command::WorkspacePackageEdition => ws_package()?
            .edition
            .map(|e| match e {
                cargo_toml::Edition::E2015 => "2015",
                cargo_toml::Edition::E2018 => "2018",
                cargo_toml::Edition::E2021 => "2021",
            })
            .unwrap_or_default()
            .to_string(),
        cli::Command::WorkspacePackageHomepage => ws_package()?.homepage.unwrap_or_default(),
        cli::Command::WorkspacePackageKeywords => ws_package()?
            .keywords
            .unwrap_or_default()
            .join(&delim_string),
        cli::Command::WorkspacePackageLicense => ws_package()?.license.unwrap_or_default(),
        cli::Command::WorkspacePackageDescription => ws_package()?.description.unwrap_or_default(),
        cli::Command::WorkspacePackageCategories => ws_package()?
            .categories
            .unwrap_or_default()
            .join(&delim_string),
    };

    println!("{}", output);

    // if let Err(err) = output(&matches, manifest) {
    //     eprintln!("Error: {}", err);
    //     std::process::exit(1);
    // }

    Ok(())
}

// Remove get argument in order to make it work with or without `get` subcommand
fn get_args() -> Vec<String> {
    let mut args: Vec<_> = std::env::args().collect();

    if args.get(1) == Some(&"get".to_owned()) {
        args.remove(1);
    }

    args
}
//
// pub fn output(matches: &ArgMatches, manifest: Manifest) -> Result<(), Box<dyn Error>> {
//     let package = || manifest.package.clone().ok_or(NotFound("package"));
//     let workspace = || manifest.workspace.clone().ok_or(NotFound("workspace"));
//     let ws_package = || workspace().and_then(|ws| ws.package.ok_or(NotFound("workspace.package")));
//
//     let delimiter: Delimiter = matches
//         .value_of("delimiter")
//         .map(|s| s.parse().unwrap())
//         .unwrap_or_default();
//
//     let delim_string = delimiter.to_string();
//
//     if let Some(version) = matches.subcommand_matches("package.version") {
//         let v: semver::Version = package()?
//             .version
//             .get()
//             .or(Err(InheritanceError("package.version")))?
//             .parse()
//             .map_err(InvalidSemver)?;
//
//         match_version(version, v, &delimiter)?;
//     }
//
//     if matches.is_present("name") {
//         println!("{}", package()?.name);
//     } else if matches.is_present("homepage") {
//         println!(
//             "{}",
//             package()?
//                 .homepage
//                 .unwrap_or_default()
//                 .get()
//                 .or(Err(InheritanceError("package.homepage")))?
//         );
//     } else if matches.is_present("license") {
//         println!(
//             "{}",
//             package()?
//                 .license
//                 .unwrap_or_default()
//                 .get()
//                 .or(Err(InheritanceError("package.license")))?
//         );
//     } else if matches.is_present("description") {
//         println!(
//             "{}",
//             package()?
//                 .description
//                 .unwrap_or_default()
//                 .get()
//                 .or(Err(InheritanceError("package.description")))?
//         );
//     } else if matches.is_present("links") {
//         println!("{}", package()?.links.unwrap_or_default());
//     } else if matches.is_present("authors") {
//         println!(
//             "{}",
//             package()?
//                 .authors
//                 .get()
//                 .or(Err(InheritanceError("package.authors")))?
//                 .join(&delim_string)
//         )
//     } else if matches.is_present("keywords") {
//         println!(
//             "{}",
//             package()?
//                 .keywords
//                 .get()
//                 .or(Err(InheritanceError("package.keywords")))?
//                 .join(&delim_string)
//         )
//     } else if matches.is_present("categories") {
//         println!(
//             "{}",
//             package()?
//                 .categories
//                 .get()
//                 .or(Err(InheritanceError("package.categories")))?
//                 .join(&delim_string)
//         )
//     } else if matches.is_present("edition") {
//         let edition = match package()?
//             .edition
//             .get()
//             .or(Err(InheritanceError("package.edition")))?
//         {
//             cargo_toml::Edition::E2015 => "2015",
//             cargo_toml::Edition::E2018 => "2018",
//             cargo_toml::Edition::E2021 => "2021",
//         };
//         println!("{}", edition);
//     } else if let Some(version) = matches.subcommand_matches("workspace.package.version") {
//         let v: semver::Version = ws_package()?
//             .version
//             .ok_or(NotFound("workspace.package.version"))?
//             .parse()
//             .map_err(InvalidSemver)?;
//
//         match_version(version, v, &delimiter)?;
//     }
//
//     Ok(())
// }
//
fn search_manifest_path(dir: &std::path::Path) -> Option<PathBuf> {
    let manifest = dir.join("Cargo.toml");

    if std::fs::metadata(&manifest).is_ok() {
        Some(manifest)
    } else {
        dir.parent().and_then(search_manifest_path)
    }
}
