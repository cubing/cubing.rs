use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::alg::{Alg, Move};

use super::{InvalidAlgError, KPuzzle, KPuzzleOrbitName, KTransformation};

pub type KStateData = HashMap<KPuzzleOrbitName, KStateOrbitData>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KStateOrbitData {
    pub pieces: Vec<usize>,
    pub orientation: Vec<usize>,
    pub orientation_mod: Option<Vec<usize>>,
}
#[derive(Debug, Clone)]
pub struct KState {
    pub kpuzzle: KPuzzle,
    pub state_data: Arc<KStateData>,
}

#[derive(Debug)]
struct OrientationMods<'a> {
    old: &'a Vec<usize>,
    new: Vec<usize>,
}

impl KState {
    pub fn apply_transformation(&self, transformation: &KTransformation) -> KState {
        let mut state_data = KStateData::new();
        for (orbit_name, orbit_definition) in &self.kpuzzle.definition().orbits {
            let num_pieces = orbit_definition.num_pieces;

            let self_orbit = &self.state_data[orbit_name];
            let other_orbit = &transformation.transformation_data[orbit_name];

            let mut pieces = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?
            let mut orientation = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?
            let mut orientation_mods: Option<OrientationMods> = None;

            if let Some(old_orientation_mod) = &self_orbit.orientation_mod {
                orientation_mods = Some(OrientationMods {
                    old: old_orientation_mod,
                    new: vec![0; num_pieces],
                });
                println!("{:?}", orientation_mods);
            }

            // TODO: optimization when either value is the identity.
            for i in 0..num_pieces {
                let transformation_idx = other_orbit.permutation[i];
                let mut piece_orientation_mod = orbit_definition.num_orientations;
                if let Some(orientation_mods) = &mut orientation_mods {
                    let orientation_mod = orientation_mods.old[transformation_idx];
                    orientation_mods.new[i] = orientation_mod;
                    if orientation_mod != 0 {
                        piece_orientation_mod = orientation_mod;
                    }
                }
                pieces[i] = self_orbit.pieces[transformation_idx];
                orientation[i] = (self_orbit.orientation[transformation_idx]
                    + other_orbit.orientation[i])
                    % piece_orientation_mod;
            }

            let orbit_data = KStateOrbitData {
                pieces,
                orientation,
                orientation_mod: orientation_mods.map(|orientation_mods| orientation_mods.new),
            };
            state_data.insert(orbit_name.clone(), orbit_data); // TODO: why do we need to coerce `orbit_name`?
        }

        KState {
            kpuzzle: self.kpuzzle.clone(),
            state_data: state_data.into(),
        }
    }

    pub fn apply_alg(&self, alg: &Alg) -> Result<KState, InvalidAlgError> {
        let transformation = self.kpuzzle.transformation_from_alg(alg)?;
        Ok(self.apply_transformation(&transformation))
    }

    pub fn apply_move(&self, m: &Move) -> Result<KState, InvalidAlgError> {
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
