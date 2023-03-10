// pub trait AlgNode {
//     fn invert(&self) -> dyn AlgNode;
// }

use core::fmt;

use super::{Grouping, Move};

#[derive(Debug, Clone, PartialEq)]
pub enum AlgNode {
    MoveNode(Move),
    GroupingNode(Grouping),
}

// TODO: Figure out how to use a trait instead of manually re-wrapping all the node types.
impl AlgNode {
    pub fn invert(&self) -> Self {
        match self {
            AlgNode::MoveNode(move_node) => AlgNode::MoveNode(move_node.invert()),
            AlgNode::GroupingNode(grouping) => AlgNode::GroupingNode(grouping.invert()),
        }
    }
}

// TODO: Figure out how to use a trait instead of manually re-wrapping all the node types.
impl fmt::Display for AlgNode {
    // TODO: memoize?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgNode::MoveNode(move_node) => move_node.fmt(f),
            AlgNode::GroupingNode(grouping) => grouping.fmt(f),
        }
    }
}

impl From<Move> for AlgNode {
    fn from(input: Move) -> Self {
        AlgNode::MoveNode(input)
    }
}

impl From<Grouping> for AlgNode {
    fn from(input: Grouping) -> Self {
        AlgNode::GroupingNode(input)
    }
}
