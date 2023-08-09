use std::error::Error;

#[derive(Debug)]
pub struct InheritanceError(pub &'static str);

impl std::fmt::Display for InheritanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The property {:?} is inherited from the workspace parent!",
            self.0
        )
    }
}

impl Error for InheritanceError {}

#[derive(Debug)]
pub struct NotSpecified(pub &'static str);

impl std::fmt::Display for NotSpecified {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` not specified in manifest", self.0)
    }
}

impl Error for NotSpecified {}

#[derive(Debug)]
pub struct InvalidSemver(pub semver::Error);

impl std::fmt::Display for InvalidSemver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid semver: {}", self.0)
    }
}

impl Error for InvalidSemver {}
