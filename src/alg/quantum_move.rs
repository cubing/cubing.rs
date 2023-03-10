use std::fmt;

use nom::{
    bytes::complete::take_while,
    combinator::{all_consuming, map_res},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
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

fn from_decimal(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn is_decimal_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_decimal(input: &str) -> IResult<&str, u32> {
    map_res(take_while(is_decimal_digit), from_decimal)(input)
}

impl From<u32> for MoveLayer {
    fn from(layer: u32) -> Self {
        MoveLayer { layer }
    }
}

impl TryFrom<&str> for MoveLayer {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match all_consuming(parse_decimal)(input) {
            Ok((_, move_layer)) => Ok(move_layer.into()),
            Err(_) => Err("Invalid move layer".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumMove {
    pub family: String,
    // TODO: prevent setting outer layer without inner layer
    pub layers: Option<MovePrefix>,
}

impl fmt::Display for QuantumMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.layers {
            Some(layers) => layers.fmt(f)?,
            None => (),
        };
        write!(f, "{}", self.family)
    }
}

// pub fn parseQuantumMove();

impl QuantumMove {
    pub fn new(family: impl Into<String>, layers: Option<MovePrefix>) -> Self {
        Self {
            family: family.into(),
            layers,
        }
    }
}
