use std::fmt;
use std::rc::Rc;

use super::amount::fmt_amount;
use super::Alg;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub alg: Rc<Alg>,
    pub amount: i32,
}

impl Grouping {
    pub fn invert(&self) -> Grouping {
        Grouping {
            alg: self.alg.clone(),
            amount: -self.amount,
        }
    }
}

impl fmt::Display for Grouping {
    // TODO: memoize?
    // TODO: dedup with `Move`?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.alg)?;
        fmt_amount(f, self.amount)
    }
}
