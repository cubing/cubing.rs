use serde::{Deserialize, Serialize};

use std::{collections::HashMap, rc::Rc};

use super::{state::KStateData, transformation::KTransformationData};

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
    // TODO: Use `Move` as the key?
    pub orbits: HashMap<String, KPuzzleOrbitDefinition>,
    pub start_state_data: Rc<KStateData>,
    // TODO: Use `Move` as the key?
    pub moves: HashMap<String, Rc<KTransformationData>>,
    pub experimental_derived_moves: Option<HashMap<String, String>>,
}
