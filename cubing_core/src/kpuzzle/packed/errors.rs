use std::fmt::Display;

/// An error due to the structure of a [`KPattern`][`super::KPattern`] (such as invalid source JSON).
#[derive(Debug)]
pub struct InvalidKPatternDataError {
    pub description: String,
}

// TODO: is Rust smart enough to optimize this using just the `From<&str>` Pattern?
impl From<String> for InvalidKPatternDataError {
    fn from(description: String) -> Self {
        Self { description }
    }
}

impl From<&str> for InvalidKPatternDataError {
    fn from(description: &str) -> Self {
        Self {
            description: description.to_owned(),
        }
    }
}

impl Display for InvalidKPatternDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

/// An error due to the structure of a [`KTransformation`][`super::KTransformation`] (such as invalid source JSON).
#[derive(Debug)]
pub struct InvalidKTransformationDataError {
    pub description: String,
}

// TODO: is Rust smart enough to optimize this using just the `From<&str>` Pattern?
impl From<String> for InvalidKTransformationDataError {
    fn from(description: String) -> Self {
        Self { description }
    }
}

impl From<&str> for InvalidKTransformationDataError {
    fn from(description: &str) -> Self {
        Self {
            description: description.to_owned(),
        }
    }
}

impl Display for InvalidKTransformationDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
