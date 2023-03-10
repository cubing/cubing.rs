use std::fmt;

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
impl From<MoveRange> for MovePrefix {
    fn from(range: MoveRange) -> Self {
        MovePrefix::Range(range)
    }
}

// TODO: Can we avoid boilerplayer for these `From` implementations?
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
