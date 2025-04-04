use std::fmt;

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveLayer {
    pub layer: u32,
}

impl MoveLayer {
    pub fn new(layer: u32) -> Self {
        Self { layer }
    }
}

impl fmt::Display for MoveLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.layer)
    }
}

impl From<u32> for MoveLayer {
    fn from(layer: u32) -> Self {
        MoveLayer { layer }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveRange {
    pub outer_layer: u32,
    pub inner_layer: u32,
}

impl MoveRange {
    pub fn new(outer_layer: u32, inner_layer: u32) -> Self {
        Self {
            outer_layer,
            inner_layer,
        }
    }
}

impl fmt::Display for MoveRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.outer_layer, self.inner_layer)
    }
}

impl From<(u32, u32)> for MoveRange {
    fn from(layers: (u32, u32)) -> Self {
        let (outer_layer, inner_layer) = layers;
        MoveRange {
            outer_layer,
            inner_layer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovePrefix {
    Layer(MoveLayer),
    Range(MoveRange),
}

impl fmt::Display for MovePrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Can we avoid a `match` without recursion?
        match self {
            MovePrefix::Layer(layer) => layer.fmt(f),
            MovePrefix::Range(range) => range.fmt(f),
        }
    }
}

// TODO: Can we avoid boilerplayer for these `From` implementations?
impl From<MoveLayer> for MovePrefix {
    fn from(layer: MoveLayer) -> Self {
        MovePrefix::Layer(layer)
    }
}
impl From<u32> for MovePrefix {
    fn from(layer: u32) -> Self {
        MovePrefix::Layer(layer.into())
    }
}
impl From<MoveRange> for MovePrefix {
    fn from(range: MoveRange) -> Self {
        MovePrefix::Range(range)
    }
}
impl From<(u32, u32)> for MovePrefix {
    fn from(layers: (u32, u32)) -> Self {
        MovePrefix::Range(layers.into())
    }
}

// TODO: Use type bounds: https://github.com/rust-lang/rust/issues/52662
impl From<MoveLayer> for Option<MovePrefix> {
    fn from(layer: MoveLayer) -> Self {
        Some(MovePrefix::Layer(layer))
    }
}
impl From<MoveRange> for Option<MovePrefix> {
    fn from(range: MoveRange) -> Self {
        Some(MovePrefix::Range(range))
    }
}

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuantumMove {
    pub family: String,
    // TODO: prevent setting outer layer without inner layer
    pub prefix: Option<MovePrefix>,
}

impl fmt::Display for QuantumMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(layers) = &self.prefix {
            layers.fmt(f)?
        };
        write!(f, "{}", self.family)
    }
}

impl QuantumMove {
    pub fn new(family: impl Into<String>, layers: Option<MovePrefix>) -> Self {
        Self {
            family: family.into(),
            prefix: layers,
        }
    }
}

struct QuantumMoveVisitor;

impl Visitor<'_> for QuantumMoveVisitor {
    type Value = QuantumMove;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let quantum_move = s.parse::<QuantumMove>();
        match quantum_move {
            Ok(quantum_move) => Ok(quantum_move),
            Err(_) => Err(serde::de::Error::invalid_value(Unexpected::Str(s), &self)),
        }
    }
}

// TODO: use https://docs.rs/serde_with/1.6.0/serde_with/index.html ?
impl Serialize for QuantumMove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for QuantumMove {
    fn deserialize<D>(deserializer: D) -> Result<QuantumMove, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(QuantumMoveVisitor)
    }
}
