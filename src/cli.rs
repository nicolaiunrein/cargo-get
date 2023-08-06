use std::{error::Error, path::PathBuf};

use clap::{Args, Parser, Subcommand};

use crate::delimiter::Delimiter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand, name = "get")]
    pub command: Command,

    #[clap(long = "entry", value_name = "PATH")]
    pub root: Option<PathBuf>,

    #[clap(
        global = true,
        long = "delimiter",
        value_name = "Tab | CR | LF | CRLF | String",
        help = "specify delimiter for list values"
    )]
    pub delimiter: Option<Delimiter>,
}

#[derive(Args, Clone)]
#[group(required = false)]
pub struct Version {
    #[arg(long, conflicts_with_all = ["pretty"])]
    full: bool,

    #[arg(long, conflicts_with_all = ["full"])]
    pretty: bool,

    #[arg(long, conflicts_with_all = ["pretty", "full"])]
    major: bool,

    #[arg(long, conflicts_with_all = ["pretty", "full"])]
    minor: bool,

    #[arg(long, conflicts_with_all = ["pretty", "full"])]
    patch: bool,

    #[arg(long, conflicts_with_all = ["pretty", "full"])]
    build: bool,

    #[arg(long, conflicts_with_all = ["pretty", "full"])]
    pre: bool,
}

impl Version {
    pub fn match_version(
        &self,
        v: semver::Version,
        delimiter: &Delimiter,
    ) -> Result<String, Box<dyn Error>> {
        if self.full {
            return Ok(v.to_string());
        }

        if self.pretty {
            return Ok(format!("v{}", v));
        }

        let mut out = Vec::new();
        let delim_string = delimiter.to_string();

        if self.major {
            out.push(v.major.to_string());
        }

        if self.minor {
            out.push(v.minor.to_string());
        }

        if self.patch {
            out.push(v.patch.to_string())
        }

        if self.build {
            out.push(v.build.to_string());
        }

        if self.pre {
            out.push(v.pre.to_string())
        }

        if out.is_empty() {
            out.push(v.to_string());
        }

        Ok(out.join(&delim_string))
    }
}

#[derive(Subcommand)]
pub enum Command {
    // package
    #[clap(name = "package.version")]
    PackageVersion {
        #[clap(flatten)]
        inner: Version,
    },
    #[clap(name = "package.authors")]
    PackageAuthors,
    #[clap(name = "package.edition")]
    PackageEdition,
    #[clap(name = "package.name")]
    PackageName,
    #[clap(name = "package.homepage")]
    PackageHomepage,
    #[clap(name = "package.keywords")]
    PackageKeywords,
    #[clap(name = "package.license")]
    PackageLicense,
    #[clap(name = "package.links")]
    PackageLinks,
    #[clap(name = "package.description")]
    PackageDescription,
    #[clap(name = "package.categories")]
    PackageCategories,

    // workspace
    #[clap(name = "workspace.members")]
    WorkspaceMembers,

    #[clap(name = "workspace.package.version")]
    #[group(required = false, multiple = false)]
    WorkspacePackageVersion {
        #[clap(flatten)]
        inner: Version,
    },
    #[clap(name = "workspace.package.authors")]
    WorkspacePackageAuthors,
    #[clap(name = "workspace.package.edition")]
    WorkspacePackageEdition,
    #[clap(name = "workspace.package.homepage")]
    WorkspacePackageHomepage,
    #[clap(name = "workspace.package.keywords")]
    WorkspacePackageKeywords,
    #[clap(name = "workspace.package.license")]
    WorkspacePackageLicense,
    #[clap(name = "workspace.package.description")]
    WorkspacePackageDescription,
    #[clap(name = "workspace.package.categories")]
    WorkspacePackageCategories,
}
