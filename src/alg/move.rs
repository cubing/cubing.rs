use std::rc::Rc;

use std::fmt;

use crate::alg::amount::fmt_amount;
use crate::alg::QuantumMove;

use super::amount::Amount;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub quantum: Rc<QuantumMove>,
    pub amount: Amount,
}

impl Move {
    pub fn invert(&self) -> Move {
        Self {
            quantum: Rc::clone(&self.quantum),
            amount: -self.amount,
        }
    }
}

impl fmt::Display for Move {
    // TODO: memoize?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.quantum)?;
        fmt_amount(f, self.amount)
    }
}
