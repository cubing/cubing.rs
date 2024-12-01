use std::fmt;
use std::sync::Arc;

use super::amount::fmt_amount;
use super::special_notation::{D_SQ_quantum, U_SQ_quantum};
use super::{Alg, AlgNode, Move};

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub alg: Arc<Alg>,
    pub amount: i32,
}

impl Grouping {
    pub fn invert(&self) -> Grouping {
        if let Some((move_0, move_1)) = self.square1_tuple() {
            let nodes = vec![move_0.invert().into(), move_1.invert().into()];
            let alg = Arc::new(Alg { nodes });
            return Self { alg, amount: 1 };
        }
        Grouping {
            alg: self.alg.clone(),
            amount: -self.amount,
        }
    }

    fn square1_tuple(&self) -> Option<(&Move, &Move)> {
        if self.alg.nodes.len() == 2 && self.amount == 1 {
            // Square-1 notation
            if let AlgNode::MoveNode(move_0) = &self.alg.nodes[0] {
                if move_0.quantum == U_SQ_quantum() {
                    if let AlgNode::MoveNode(move_1) = &self.alg.nodes[1] {
                        if move_1.quantum == D_SQ_quantum() {
                            return Some((move_0, move_1));
                        }
                    }
                }
            }
        }
        None
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
        } else if let Some((move_0, move_1)) = self.square1_tuple() {
            // Square-1 notation
            return write!(f, "({}, {})", move_0.amount, move_1.amount);
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

#[cfg(test)]
mod tests {
    use crate::alg::Alg;

    #[test]
    fn square1_tuple_inversion() {
        let alg = "(3, -4)".parse::<Alg>().unwrap();
        assert_eq!(alg.invert(), "(-3, 4)".parse::<Alg>().unwrap());
    }
}
