use std::fmt;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LineComment {
    text: String,
}

impl LineComment {
    // What's the best pattern to ensure that `text` can't contain newline?
    // (`try_from` should strip the leading `//`, so it's not the same)
    pub fn try_new(s: &str) -> Result<LineComment, String> {
        if s.contains('\n') {
            return Err("Line comment cannot contain a newline.".into());
        }
        Ok(LineComment { text: s.to_owned() })
    }

    pub fn invert(&self) -> LineComment {
        self.clone()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl fmt::Display for LineComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Note: does NOT include newline, in case it's the final node in an alg.
        write!(f, "//{}", self.text)
    }
}
