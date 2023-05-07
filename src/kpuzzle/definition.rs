use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::alg::{Alg, Move};

use super::{state::KStateData, transformation::KTransformationData};

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

// use super::super::{state::KStateData, transformation::KTransformationData};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KPuzzleOrbitDefinition {
    pub num_pieces: usize,       // TODO
    pub num_orientations: usize, // TODO
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KPuzzleDefinition {
    pub name: String,
    pub orbits: HashMap<KPuzzleOrbitName, KPuzzleOrbitDefinition>,
    pub start_state_data: Rc<KStateData>,
    pub moves: HashMap<Move, Rc<KTransformationData>>,
    pub experimental_derived_moves: Option<HashMap<Move, Alg>>,
}
