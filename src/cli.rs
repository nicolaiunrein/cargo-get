use std::{error::Error, path::PathBuf};

use clap::{Args, Parser, Subcommand};

use crate::{delimiter::Delimiter, terminator::Terminator};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand, name = "get")]
    pub command: Command,

    #[clap(
        global = true,
        long,
        value_name = "PATH",
        help = "Path to the crate root to query"
    )]
    pub entry: Option<PathBuf>,

    #[clap(
        global = true,
        long,
        value_name = "Tab | CR | LF | CRLF | String",
        help = "Delimiter for array values"
    )]
    pub delimiter: Option<Delimiter>,

    #[clap(
        global = true,
        long,
        value_name = "CR | LF | CRLF | Nul | String",
        help = "String terminator for the output that is returned"
    )]
    pub terminator: Option<Terminator>,
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
            return Ok(format!("v{v}"));
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
    // **************** package ****************
    #[clap(name = "package.name")]
    PackageName,

    #[clap(name = "package.edition")]
    PackageEdition,

    #[clap(name = "package.rust_version")]
    PackageRustVersion,

    #[clap(name = "package.version")]
    PackageVersion {
        #[clap(flatten)]
        inner: Version,
    },

    #[clap(name = "package.build")]
    PackageBuild,

    #[clap(name = "package.workspace")]
    PackageWorkspace,

    #[clap(name = "package.authors")]
    PackageAuthors,

    #[clap(name = "package.links")]
    PackageLinks,

    #[clap(name = "package.description")]
    PackageDescription,

    #[clap(name = "package.homepage")]
    PackageHomepage,

    #[clap(name = "package.readme")]
    PackageReadme,

    #[clap(name = "package.keywords")]
    PackageKeywords,

    #[clap(name = "package.categories")]
    PackageCategories,

    #[clap(name = "package.exclude")]
    PackageExclude,

    #[clap(name = "package.include")]
    PackageInclude,

    #[clap(name = "package.license")]
    PackageLicense,

    #[clap(name = "package.license_file")]
    PackageLicenseFile,

    #[clap(name = "package.repository")]
    PackageRepository,

    #[clap(name = "package.default_run")]
    PackageDefaultRun,

    #[clap(name = "package.publish")]
    PackagePublish,

    #[clap(name = "package.resolver")]
    PackageResolver,

    #[clap(name = "package.metadata")]
    PackageMetadata,

    // **************** workspace ****************
    #[clap(name = "workspace.members")]
    WorkspaceMembers,

    #[clap(name = "workspace.default_members")]
    WorkspaceDefaultMembers,

    #[clap(name = "workspace.package.authors")]
    WorkspacePackageAuthors,

    #[clap(name = "workspace.package.categories")]
    WorkspacePackageCategories,

    #[clap(name = "workspace.package.description")]
    WorkspacePackageDescription,

    #[clap(name = "workspace.package.documentation")]
    WorkspacePackageDocumentation,

    #[clap(name = "workspace.package.edition")]
    WorkspacePackageEdition,

    #[clap(name = "workspace.package.exclude")]
    WorkspacePackageExclude,

    #[clap(name = "workspace.package.homepage")]
    WorkspacePackageHomepage,

    #[clap(name = "workspace.package.include")]
    WorkspacePackageInclude,

    #[clap(name = "workspace.package.keywords")]
    WorkspacePackageKeywords,

    #[clap(name = "workspace.package.license")]
    WorkspacePackageLicense,

    #[clap(name = "workspace.package.license_file")]
    WorkspacePackageLicenseFile,

    #[clap(name = "workspace.package.publish")]
    WorkspacePackagePublish,

    #[clap(name = "workspace.package.readme")]
    WorkspacePackageReadme,

    #[clap(name = "workspace.package.repository")]
    WorkspacePackageRepository,

    #[clap(name = "workspace.package.rust_version")]
    WorkspacePackageRustVersion,

    #[clap(name = "workspace.package.version")]
    #[group(required = false, multiple = false)]
    WorkspacePackageVersion {
        #[clap(flatten)]
        inner: Version,
    },
}
