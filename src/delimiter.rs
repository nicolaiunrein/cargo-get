use std::fmt;
pub enum Delimiter {
    Tab,
    CR,
    LF,
    CRLF,
    String(String),
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Tab => write!(f, "\t"),
            Self::CR => write!(f, "\r"),
            Self::LF => write!(f, "\n"),
            Self::CRLF => write!(f, "\r\n"),
            Self::String(s) => write!(f, "{}", s),
        }
    }
}

impl std::str::FromStr for Delimiter {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Tab" => Ok(Self::Tab),
            "CR" => Ok(Self::CR),
            "LF" => Ok(Self::LF),
            "CRLF" => Ok(Self::CRLF),
            other => Ok(Self::String(other.to_owned())),
        }
    }
}

impl Default for Delimiter {
    fn default() -> Self {
        Self::CRLF
    }
}
