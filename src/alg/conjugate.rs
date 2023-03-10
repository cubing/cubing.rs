use std::fmt;
use std::rc::Rc;

use super::Alg;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Clone, PartialEq)]
pub struct Conjugate {
    pub a: Rc<Alg>,
    pub b: Rc<Alg>,
}

impl Conjugate {
    pub fn invert(&self) -> Conjugate {
        Conjugate {
            a: self.a.clone(),
            b: self.b.invert().into(),
        }
    }
}

impl fmt::Display for Conjugate {
    // TODO: memoize?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {}]", self.a, self.b)
    }
}
