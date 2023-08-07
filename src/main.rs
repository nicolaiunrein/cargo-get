mod cli;
mod delimiter;
mod error;

use cargo_toml::Manifest;
use clap::Parser;
use delimiter::Delimiter;
use error::NotSpecified;
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<_> = std::env::args().collect();

    if args.get(1) == Some(&"get".to_owned()) {
        args.remove(1);
    }

    let cli = cli::Cli::parse_from(args);

    match output(cli) {
        Ok(out) => println!("{}", out),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}

pub fn output(cli: cli::Cli) -> Result<String, Box<dyn Error>> {
    let entry_point = match cli.entry.clone() {
        Some(p) => p,
        None => std::env::current_dir()?,
    };

    let entry_point_absolute =
        std::fs::canonicalize(entry_point).map_err(|_| "No such file or directory")?;

    let manifest_path = search_manifest_path(&entry_point_absolute).ok_or("No manifest found")?;

    let manifest = Manifest::from_path(&manifest_path)?;

    let package = || manifest.package.clone().ok_or(NotSpecified("package"));
    let workspace = || manifest.workspace.clone().ok_or(NotSpecified("workspace"));
    let ws_package =
        || workspace().and_then(|ws| ws.package.ok_or(NotSpecified("workspace.package")));

    let delimiter: Delimiter = cli.delimiter.unwrap_or_default();
    let delim_string = delimiter.to_string();

    let output = match cli.command {
        cli::Command::PackageVersion { inner } => {
            let v: semver::Version = package()?.version().parse()?;
            inner.match_version(v, &delimiter)?
        }
        cli::Command::PackageAuthors => package()?.authors().join(&delim_string),

        cli::Command::PackageEdition => match package()?.edition() {
            cargo_toml::Edition::E2015 => "2015",
            cargo_toml::Edition::E2018 => "2018",
            cargo_toml::Edition::E2021 => "2021",
        }
        .to_string(),
        cli::Command::PackageName => package()?.name().to_string(),
        cli::Command::PackageHomepage => package()?
            .homepage()
            .ok_or_else(|| NotSpecified("package.homepage"))?
            .to_string(),
        cli::Command::PackageKeywords => package()?.keywords().join(&delim_string),
        cli::Command::PackageLicense => package()?
            .license()
            .ok_or_else(|| NotSpecified("package.license"))?
            .to_string(),

        cli::Command::PackageLinks => package()?
            .links()
            .ok_or_else(|| NotSpecified("package.links"))?
            .to_string(),
        cli::Command::PackageDescription => {
            package()?.description().unwrap_or_default().to_string()
        }
        cli::Command::PackageCategories => package()?.categories().join(&delim_string),

        cli::Command::PackageRustVersion => package()?
            .rust_version()
            .ok_or_else(|| NotSpecified("package.rust_version"))?
            .to_string(),
        cli::Command::PackageBuild => package()?
            .build
            .ok_or_else(|| NotSpecified("package.build"))?
            .as_path()
            .unwrap()
            .to_string_lossy()
            .to_string(),

        cli::Command::PackageWorkspace => package()?
            .workspace
            .ok_or_else(|| NotSpecified("package.workspace"))?
            .to_string(),

        cli::Command::PackageReadme => package()?
            .readme()
            .as_path()
            .ok_or_else(|| NotSpecified("package.readme"))?
            .to_string_lossy()
            .to_string(),

        cli::Command::PackageExclude => package()?.exclude().join(&delim_string),
        cli::Command::PackageInclude => package()?.include().join(&delim_string),
        cli::Command::PackageLicenseFile => package()?
            .license_file()
            .ok_or_else(|| NotSpecified("package.license_file"))?
            .to_string_lossy()
            .to_string(),

        cli::Command::PackageRepository => package()?
            .repository()
            .ok_or_else(|| NotSpecified("package.repository"))?
            .to_string(),

        cli::Command::PackageDefaultRun => package()?
            .default_run
            .ok_or_else(|| NotSpecified("package.default_run"))?
            .to_string(),

        cli::Command::PackagePublish => match package()?.publish() {
            cargo_toml::Publish::Flag(flag) => flag.to_string(),
            cargo_toml::Publish::Registry(list) => list.join(&delim_string),
        },
        cli::Command::PackageResolver => package()?
            .resolver
            .ok_or_else(|| NotSpecified("package.resolver"))?
            .to_string(),

        cli::Command::PackageMetadata => package()?
            .metadata
            .ok_or_else(|| NotSpecified("package.metadata"))?
            .to_string(),

        cli::Command::WorkspaceMembers => workspace()?.members.join(&delim_string),

        cli::Command::WorkspacePackageVersion { inner } => {
            let v: semver::Version = ws_package()?
                .version
                .ok_or_else(|| NotSpecified("workspace.package.version"))?
                .parse()?;
            inner.match_version(v, &delimiter)?
        }

        cli::Command::WorkspacePackageAuthors => ws_package()?
            .authors
            .ok_or_else(|| NotSpecified("workspace.package.authors"))?
            .join(&delim_string),

        cli::Command::WorkspacePackageEdition => ws_package()?
            .edition
            .map(|e| match e {
                cargo_toml::Edition::E2015 => "2015",
                cargo_toml::Edition::E2018 => "2018",
                cargo_toml::Edition::E2021 => "2021",
            })
            .ok_or_else(|| NotSpecified("workspace.package.edition"))?
            .to_string(),

        cli::Command::WorkspacePackageHomepage => ws_package()?
            .homepage
            .ok_or_else(|| NotSpecified("workspace.package.homepage"))?,

        cli::Command::WorkspacePackageKeywords => ws_package()?
            .keywords
            .ok_or_else(|| NotSpecified("workspace.package.keywords"))?
            .join(&delim_string),

        cli::Command::WorkspacePackageLicense => ws_package()?
            .license
            .ok_or_else(|| NotSpecified("workspace.package.license"))?,

        cli::Command::WorkspacePackageDescription => ws_package()?
            .description
            .ok_or_else(|| NotSpecified("workspace.package.license"))?,

        cli::Command::WorkspacePackageCategories => ws_package()?
            .categories
            .ok_or_else(|| NotSpecified("workspace.package.categories"))?
            .join(&delim_string),
        cli::Command::WorkspacePackageDocumentation => ws_package()?
            .documentation
            .ok_or_else(|| NotSpecified("workspace.package.documentation"))?,

        cli::Command::WorkspacePackageExclude => ws_package()?
            .exclude
            .ok_or_else(|| NotSpecified("workspace.package.exclude"))?
            .join(&delim_string),

        cli::Command::WorkspacePackageInclude => ws_package()?
            .include
            .ok_or_else(|| NotSpecified("workspace.package.include"))?
            .join(&delim_string),

        cli::Command::WorkspacePackageLicenseFile => ws_package()?
            .license_file
            .ok_or_else(|| NotSpecified("workspace.package.license_file"))?
            .to_string_lossy()
            .to_string(),

        cli::Command::WorkspacePackagePublish => match ws_package()?.publish {
            cargo_toml::Publish::Flag(flag) => flag.to_string(),
            cargo_toml::Publish::Registry(list) => list.join(&delim_string),
        },
        cli::Command::WorkspacePackageReadme => ws_package()?
            .readme
            .as_path()
            .ok_or_else(|| NotSpecified("workspace.package.readme"))?
            .to_string_lossy()
            .to_string(),

        cli::Command::WorkspacePackageRepository => ws_package()?
            .repository
            .ok_or_else(|| NotSpecified("workspace.package.repository"))?,

        cli::Command::WorkspacePackageRustVersion => ws_package()?
            .rust_version
            .ok_or_else(|| NotSpecified("workspace.package.rust_version"))?
            .to_string(),
    };

    Ok(output)
}

fn search_manifest_path(dir: &std::path::Path) -> Option<PathBuf> {
    let manifest = dir.join("Cargo.toml");

    if std::fs::metadata(&manifest).is_ok() {
        Some(manifest)
    } else {
        dir.parent().and_then(search_manifest_path)
    }
}
