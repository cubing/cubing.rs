use serde::{Deserialize, Serialize};

pub(crate) use std::{collections::HashMap, fmt::Display};

use crate::alg::{Alg, Move};

use super::{kpattern::KPatternData, ktransformation::KTransformationData};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KPuzzleOrbitName(pub String);

impl From<&str> for KPuzzleOrbitName {
    fn from(value: &str) -> Self {
        KPuzzleOrbitName(value.to_owned())
    }
}

impl Display for KPuzzleOrbitName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// use super::super::{pattern::KPatternData, transformation::KTransformationData};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KPuzzleOrbitDefinition {
    pub orbit_name: KPuzzleOrbitName,
    pub num_pieces: usize,       // TODO
    pub num_orientations: usize, // TODO
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KPuzzleDefinition {
    pub name: String,
    pub orbits: Vec<KPuzzleOrbitDefinition>,
    pub default_pattern: KPatternData,
    pub moves: HashMap<Move, KTransformationData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_moves: Option<HashMap<Move, Alg>>,
}
