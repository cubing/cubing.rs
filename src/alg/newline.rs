use std::fmt;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Newline {}

impl Newline {
    pub fn invert(&self) -> Newline {
        Newline {}
    }
}

impl fmt::Display for Newline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `writeln!` adds `\n` (no `\r`) on all platforms, so it is safe to use.
        writeln!(f)
    }
}
