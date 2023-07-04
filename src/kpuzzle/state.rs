use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::alg::{Alg, Move};

use super::{KPuzzle, KPuzzleOrbitName, KTransformation};

pub type KStateData = HashMap<KPuzzleOrbitName, KStateOrbitData>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KStateOrbitData {
    pub pieces: Vec<usize>,
    pub orientation: Vec<usize>,
}
#[derive(Debug, Clone)]
pub struct KState {
    pub kpuzzle: KPuzzle,
    pub state_data: Arc<KStateData>,
}

impl KState {
    pub fn apply_transformation(&self, transformation: &KTransformation) -> KState {
        let mut state_data = KStateData::new();
        for (orbit_name, orbit_definition) in &self.kpuzzle.definition().orbits {
            let num_pieces = orbit_definition.num_pieces;

            let mut pieces = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?
            let mut orientation = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?

            let self_orbit = &self.state_data[orbit_name];
            let other_orbit = &transformation.transformation_data[orbit_name];

            // TODO: optimization when either value is the identity.
            for i in 0..num_pieces {
                pieces[i] = self_orbit.pieces[other_orbit.permutation[i]];
                orientation[i] = (self_orbit.orientation[other_orbit.permutation[i]]
                    + other_orbit.orientation[i])
                    % orbit_definition.num_orientations;
            }

            let orbit_data = KStateOrbitData {
                pieces,
                orientation,
            };
            state_data.insert(orbit_name.clone(), orbit_data); // TODO: why do we need to coerce `orbit_name`?
        }

        KState {
            kpuzzle: self.kpuzzle.clone(),
            state_data: state_data.into(),
        }
    }

    pub fn apply_alg(&self, alg: &Alg) -> Result<KState, String> {
        let transformation = self.kpuzzle.transformation_from_alg(alg)?;
        Ok(self.apply_transformation(&transformation))
    }

    pub fn apply_move(&self, m: &Move) -> Result<KState, String> {
        let transformation = self.kpuzzle.transformation_from_move(m)?;
        Ok(self.apply_transformation(&transformation))
    }
}

impl PartialEq<KState> for KState {
    fn eq(&self, other: &Self) -> bool {
        // TODO: check if the KPuzzle comparison is correct and performant.
        self.kpuzzle == other.kpuzzle && self.state_data == other.state_data
    }
}
