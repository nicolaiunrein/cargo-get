use clap::ValueEnum;

use crate::cli::Command;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum OutputFormat {
    GithubAction,
}

impl OutputFormat {
    pub(crate) fn format_pair(&self, cmd: Command, value: &str) -> String {
        const DELIMITER: &str = "EOF";
        format!("{cmd}<<{DELIMITER}\n{value}\n{DELIMITER}\n")
    }
}
