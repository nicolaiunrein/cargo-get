use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
    Tab,
    Cr,
    Lf,
    CrLf,
    String(String),
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Tab => write!(f, "\t"),
            Self::Cr => write!(f, "\r"),
            Self::Lf => writeln!(f),
            Self::CrLf => write!(f, "\r\n"),
            Self::String(s) => write!(f, "{}", s),
        }
    }
}

impl std::str::FromStr for Delimiter {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "tab" => Ok(Self::Tab),
            "cr" => Ok(Self::Cr),
            "lf" => Ok(Self::Lf),
            "crlf" => Ok(Self::CrLf),
            _ => Ok(Self::String(s.to_owned())),
        }
    }
}

impl Default for Delimiter {
    fn default() -> Self {
        Self::CrLf
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
            Delimiter::Cr,
            Delimiter::Lf,
            Delimiter::CrLf,
            Delimiter::String("abc!@#$%^&*()".to_owned())
        );

        let expected = "\t \r \n \r\n abc!@#$%^&*()".to_owned();

        assert_eq!(res, expected);
    }

    #[test]
    fn parse_ok() {
        // Capitalized
        assert_eq!("Tab".parse::<Delimiter>(), Ok(Delimiter::Tab));
        assert_eq!("CR".parse::<Delimiter>(), Ok(Delimiter::Cr));
        assert_eq!("LF".parse::<Delimiter>(), Ok(Delimiter::Lf));
        assert_eq!("CRLF".parse::<Delimiter>(), Ok(Delimiter::CrLf));

        // Lowercase
        assert_eq!("tab".parse::<Delimiter>(), Ok(Delimiter::Tab));
        assert_eq!("cr".parse::<Delimiter>(), Ok(Delimiter::Cr));
        assert_eq!("lf".parse::<Delimiter>(), Ok(Delimiter::Lf));
        assert_eq!("crlf".parse::<Delimiter>(), Ok(Delimiter::CrLf));

        // Uppercase
        assert_eq!("TAB".parse::<Delimiter>(), Ok(Delimiter::Tab));
        assert_eq!("Cr".parse::<Delimiter>(), Ok(Delimiter::Cr));
        assert_eq!("Lf".parse::<Delimiter>(), Ok(Delimiter::Lf));
        assert_eq!("CrLf".parse::<Delimiter>(), Ok(Delimiter::CrLf));
    }
}
