use clap::ValueEnum;

use crate::cli::Command;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum OutputFormat {
    GithubAction,
    Pretty,
}

impl OutputFormat {
    pub(crate) fn format_pair(&self, cmd: Command, value: &str) -> String {
        match self {
            Self::GithubAction => {
                const DELIMITER: &str = "EOF";
                format!("{cmd}<<{DELIMITER}\n{value}\n{DELIMITER}\n")
            }
            Self::Pretty => {
                const INDENT: &str = "  ";
                let style = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Green.into()));
                let lines = value
                    .lines()
                    .map(|line| format!("{INDENT}|>{INDENT}{line}\n"))
                    .collect::<String>();

                format!("\n{INDENT}{style}{cmd}{style:#}\n{lines}")
            }
        }
    }
}
