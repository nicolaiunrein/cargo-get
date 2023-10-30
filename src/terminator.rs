use std::fmt;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Terminator {
    Cr,
    CrLf,
    #[default]
    Lf,
    Nul,
    String(String),
}

impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Cr => "\r",
            Self::CrLf => "\r\n",
            Self::Lf => "\n",
            Self::Nul => "\0",
            Self::String(s) => s,
        })
    }
}

impl std::str::FromStr for Terminator {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "cr" => Ok(Self::Cr),
            "crlf" => Ok(Self::CrLf),
            "lf" => Ok(Self::Lf),
            "nul" => Ok(Self::Nul),
            _ => Ok(Self::String(s.to_owned())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_ok() {
        let res = format!(
            "{} {} {} {} {}",
            Terminator::Cr,
            Terminator::CrLf,
            Terminator::Lf,
            Terminator::Nul,
            Terminator::String("abc!@#$%^&*()".to_owned())
        );

        let expected = "\r \r\n \n \0 abc!@#$%^&*()".to_owned();

        assert_eq!(res, expected);
    }

    #[test]
    fn parse_ok() -> Result<(), Box<dyn std::error::Error>> {
        for (input, result) in [
            ("Cr", Terminator::Cr),
            ("CrLf", Terminator::CrLf),
            ("Lf", Terminator::Lf),
            ("Nul", Terminator::Nul),
        ] {
            assert_eq!(input.parse::<Terminator>()?, result);
            assert_eq!(input.to_lowercase().parse::<Terminator>()?, result);
            assert_eq!(input.to_uppercase().parse::<Terminator>()?, result);
        }
        Ok(())
    }
}
