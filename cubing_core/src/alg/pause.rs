use std::fmt;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Pause {}

impl Pause {
    pub fn invert(&self) -> Pause {
        Pause {}
    }
}

impl fmt::Display for Pause {
    // TODO: memoize?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ".")
    }
}
