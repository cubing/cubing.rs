use std::fmt;

use super::alg_node::AlgNode;

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Alg {
    pub nodes: Vec<AlgNode>,
}

impl Alg {
    pub fn invert(&self) -> Alg {
        let nodes = self.nodes.iter().rev().map(|m| m.invert()).collect();
        Alg { nodes }
    }
}

impl fmt::Display for Alg {
    // TODO: memoize?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for node in self.nodes.iter() {
            if first {
                first = false;
            } else {
                write!(f, " ")?;
            }
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}
