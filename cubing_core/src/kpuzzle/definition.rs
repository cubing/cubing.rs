use serde::{Deserialize, Serialize};

use std::fmt::Debug;
pub(crate) use std::{collections::HashMap, fmt::Display};

use crate::alg::{Alg, Move};

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
    pub num_pieces: u8,       // TODO
    pub num_orientations: u8, // TODO
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

#[derive(
    PartialEq,
    Serialize,
    Deserialize,
    Clone, // TODO
)]
#[serde(rename_all = "camelCase")]
pub struct KPatternOrbitData {
    pub pieces: Vec<u8>,
    pub orientation: Vec<u8>,
    pub orientation_mod: Option<Vec<u8>>,
}

struct SameLineDebugVecU8<'a>(&'a Vec<u8>);

impl Debug for SameLineDebugVecU8<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Debug for KPatternOrbitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KPatternOrbitData")
            .field("pieces", &SameLineDebugVecU8(&self.pieces))
            .field("orientation", &SameLineDebugVecU8(&self.orientation))
            .field(
                "orientation_mod",
                &self.orientation_mod.as_ref().map(SameLineDebugVecU8),
            )
            .finish()
    }
}

pub type KPatternData = HashMap<KPuzzleOrbitName, KPatternOrbitData>;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KTransformationOrbitData {
    pub permutation: Vec<u8>,
    pub orientation_delta: Vec<u8>,
}

impl Debug for KTransformationOrbitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KTransformationOrbitData")
            .field("permutation", &SameLineDebugVecU8(&self.permutation))
            .field(
                "orientation_delta",
                &SameLineDebugVecU8(&self.orientation_delta),
            )
            .finish()
    }
}

// TODO: Use `Move` as the key?
pub type KTransformationData = HashMap<KPuzzleOrbitName, KTransformationOrbitData>;
