// pub trait AlgNode {
//     fn invert(&self) -> dyn AlgNode;
// }

use core::fmt;

use super::{Commutator, Conjugate, Grouping, Move, Pause};

#[derive(Debug, Clone, PartialEq)]
pub enum AlgNode {
    MoveNode(Move),
    PauseNode(Pause),
    GroupingNode(Grouping),
    CommutatorNode(Commutator),
    ConjugateNode(Conjugate),
}

// TODO: Figure out how to use a trait instead of manually re-wrapping all the node types.
impl AlgNode {
    pub fn invert(&self) -> Self {
        match self {
            AlgNode::MoveNode(move_node) => AlgNode::MoveNode(move_node.invert()),
            AlgNode::PauseNode(pause_node) => AlgNode::PauseNode(pause_node.invert()),
            AlgNode::GroupingNode(move_node) => AlgNode::GroupingNode(move_node.invert()),
            AlgNode::CommutatorNode(move_node) => AlgNode::CommutatorNode(move_node.invert()),
            AlgNode::ConjugateNode(move_node) => AlgNode::ConjugateNode(move_node.invert()),
        }
    }
}

// TODO: Figure out how to use a trait instead of manually re-wrapping all the node types.
impl fmt::Display for AlgNode {
    // TODO: memoize?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgNode::MoveNode(move_node) => move_node.fmt(f),
            AlgNode::PauseNode(pause_node) => pause_node.fmt(f),
            AlgNode::GroupingNode(move_node) => move_node.fmt(f),
            AlgNode::CommutatorNode(move_node) => move_node.fmt(f),
            AlgNode::ConjugateNode(grouping) => grouping.fmt(f),
        }
    }
}

impl From<Move> for AlgNode {
    fn from(input: Move) -> Self {
        AlgNode::MoveNode(input)
    }
}

impl From<Pause> for AlgNode {
    fn from(input: Pause) -> Self {
        AlgNode::PauseNode(input)
    }
}

impl From<Grouping> for AlgNode {
    fn from(input: Grouping) -> Self {
        AlgNode::GroupingNode(input)
    }
}

impl From<Commutator> for AlgNode {
    fn from(input: Commutator) -> Self {
        AlgNode::CommutatorNode(input)
    }
}

impl From<Conjugate> for AlgNode {
    fn from(input: Conjugate) -> Self {
        AlgNode::ConjugateNode(input)
    }
}
