use std::fmt;

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};

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

// TODO: use https://docs.rs/serde_with/1.6.0/serde_with/index.html ?
impl Serialize for Alg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Alg {
    fn deserialize<D>(deserializer: D) -> Result<Alg, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AlgVisitor)
    }
}

struct AlgVisitor;

impl Visitor<'_> for AlgVisitor {
    type Value = Alg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let alg = s.parse::<Alg>();
        match alg {
            Ok(alg) => Ok(alg),
            Err(_) => Err(serde::de::Error::invalid_value(Unexpected::Str(s), &self)),
        }
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
