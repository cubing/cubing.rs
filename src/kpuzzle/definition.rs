use std::{collections::HashMap, rc::Rc};

use super::{state::KStateData, transformation::KTransformationData};

// use super::super::{state::KStateData, transformation::KTransformationData};

pub struct KPuzzleOrbitDefinition {
    pub num_pieces: u32,
    pub num_orientations: u32,
}
pub struct KPuzzleDefinition {
    pub name: String,
    pub orbits: HashMap<String, KPuzzleOrbitDefinition>,
    pub start_state_data: KStateData,
    // TODO: Use `Move` as the key?
    pub moves: HashMap<String, Rc<KTransformationData>>,
    // experimentalDerivedMoves?: Record<string, string>;
}
