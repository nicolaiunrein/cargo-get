use std::fmt;

#[derive(Debug, PartialEq, Clone)]
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
            Self::LF => writeln!(f),
            Self::CRLF => write!(f, "\r\n"),
            Self::String(s) => write!(f, "{}", s),
        }
    }
}

impl std::str::FromStr for Delimiter {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "tab" => Ok(Self::Tab),
            "cr" => Ok(Self::CR),
            "lf" => Ok(Self::LF),
            "crlf" => Ok(Self::CRLF),
            _ => Ok(Self::String(s.to_owned())),
        }
    }
}

impl Default for Delimiter {
    fn default() -> Self {
        Self::CRLF
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_ok() {
        let res = format!(
            "{} {} {} {} {}",
            Delimiter::Tab,
            Delimiter::CR,
            Delimiter::LF,
            Delimiter::CRLF,
            Delimiter::String("abc!@#$%^&*()".to_owned())
        );

        let expected = "\t \r \n \r\n abc!@#$%^&*()".to_owned();

        assert_eq!(res, expected);
    }

    #[test]
    fn parse_ok() {
        // Capitalized
        assert_eq!("Tab".parse::<Delimiter>(), Ok(Delimiter::Tab));
        assert_eq!("CR".parse::<Delimiter>(), Ok(Delimiter::CR));
        assert_eq!("LF".parse::<Delimiter>(), Ok(Delimiter::LF));
        assert_eq!("CRLF".parse::<Delimiter>(), Ok(Delimiter::CRLF));

        // Lowercase
        assert_eq!("tab".parse::<Delimiter>(), Ok(Delimiter::Tab));
        assert_eq!("cr".parse::<Delimiter>(), Ok(Delimiter::CR));
        assert_eq!("lf".parse::<Delimiter>(), Ok(Delimiter::LF));
        assert_eq!("crlf".parse::<Delimiter>(), Ok(Delimiter::CRLF));

        // Uppercase
        assert_eq!("TAB".parse::<Delimiter>(), Ok(Delimiter::Tab));
        assert_eq!("CR".parse::<Delimiter>(), Ok(Delimiter::CR));
        assert_eq!("LF".parse::<Delimiter>(), Ok(Delimiter::LF));
        assert_eq!("CRLF".parse::<Delimiter>(), Ok(Delimiter::CRLF));
    }
}
