use std::fmt;
use std::rc::Rc;

use super::Alg;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Clone, PartialEq)]
pub struct Commutator {
    pub a: Rc<Alg>,
    pub b: Rc<Alg>,
}

impl Commutator {
    pub fn invert(&self) -> Commutator {
        Commutator {
            a: self.b.clone(),
            b: self.a.clone(),
        }
    }
}

impl fmt::Display for Commutator {
    // TODO: memoize?
    // TODO: dedup with `Move`?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.a, self.b)
    }
}
