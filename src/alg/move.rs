use std::rc::Rc;

use std::fmt;

use crate::alg::QuantumMove;

// TODO: figure out whether to hash the string
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub quantum: Rc<QuantumMove>,
    pub amount: i32,
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
        if self.amount == 1 {
            write!(f, "{}", self.quantum)
        } else if self.amount == -1 {
            write!(f, "{}'", self.quantum)
        } else if self.amount < 0 {
            write!(f, "{}{}'", self.quantum, -self.amount)
        } else {
            write!(f, "{}{}", self.quantum, self.amount)
        }
    }
}
