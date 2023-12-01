use std::fmt;
use std::sync::Arc;

use super::amount::fmt_amount;
use super::Alg;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub alg: Arc<Alg>,
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
        let mut include_parentheses = true;
        if self.alg.nodes.len() == 1 {
            include_parentheses = !matches!(
                self.alg.nodes[0],
                super::AlgNode::CommutatorNode(_) | super::AlgNode::ConjugateNode(_)
            )
        }
        if include_parentheses {
            write!(f, "(")?;
        }
        write!(f, "{}", self.alg)?;
        if include_parentheses {
            write!(f, ")")?;
        }
        fmt_amount(f, self.amount)
    }
}
