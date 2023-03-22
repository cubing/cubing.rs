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
        let mut previous: Option<&AlgNode> = None;
        for current in self.nodes.iter() {
            if let Some(previous) = previous {
                write!(f, "{}", space_between(previous, current))?;
            }
            write!(f, "{}", current)?;
            previous = Some(current);
        }
        Ok(())
    }
}

fn space_between(u1: &AlgNode, u2: &AlgNode) -> &'static str {
    match (u1, u2) {
        (AlgNode::LineCommentNode(_), AlgNode::NewlineNode(_)) => "",
        (AlgNode::LineCommentNode(_), _) => "\n",
        (AlgNode::NewlineNode(_), _) => "",
        (_, AlgNode::NewlineNode(_)) => "",
        (_, _) => " ",
    }
}
