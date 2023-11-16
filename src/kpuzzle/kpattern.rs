use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::alg::{Alg, Move};

use super::{KPuzzleOrbitName, UnpackedInvalidAlgError, UnpackedKPuzzle, UnpackedKTransformation};

pub type KPatternData = HashMap<KPuzzleOrbitName, KPatternOrbitData>;

#[derive(
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Clone, // TODO
)]
#[serde(rename_all = "camelCase")]
pub struct KPatternOrbitData {
    pub pieces: Vec<usize>,
    pub orientation: Vec<usize>,
    pub orientation_mod: Option<Vec<usize>>,
}
#[derive(Debug, Clone)]
pub struct UnpackedKPattern {
    pub kpuzzle: UnpackedKPuzzle,
    pub kpattern_data: Arc<KPatternData>,
}

#[derive(Debug)]
struct OrientationMods<'a> {
    old: &'a Vec<usize>,
    new: Vec<usize>,
}

impl UnpackedKPattern {
    pub fn apply_transformation(
        &self,
        transformation: &UnpackedKTransformation,
    ) -> UnpackedKPattern {
        let mut pattern_data = KPatternData::new();
        for orbit_definition in &self.kpuzzle.definition().orbits {
            let num_pieces = orbit_definition.num_pieces;

            let self_orbit = &self.kpattern_data[&orbit_definition.orbit_name];
            let other_orbit = &transformation.ktransformation_data[&orbit_definition.orbit_name];

            // TODO: figure out the fastest way to populate the vectors.
            // So far, initializing all entries to 0 is measurably faster than using `Vec::with_capacity(…)` and `.push(…)`.
            // However, there might be a way to avoid setting the entries to 0 (which would avoid unneeded work, since they will all be overwritten).
            let mut pieces = vec![0; num_pieces];
            let mut orientation = vec![0; num_pieces];
            let mut orientation_mods: Option<OrientationMods> = None;

            if let Some(old_orientation_mod) = &self_orbit.orientation_mod {
                orientation_mods = Some(OrientationMods {
                    old: old_orientation_mod,
                    new: vec![0; num_pieces],
                });
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
                    + other_orbit.orientation_delta[i])
                    % piece_orientation_mod;
            }

            let orbit_data = KPatternOrbitData {
                pieces,
                orientation,
                orientation_mod: orientation_mods.map(|orientation_mods| orientation_mods.new),
            };
            pattern_data.insert(orbit_definition.orbit_name.clone(), orbit_data);
            // TODO: why do we need to coerce `orbit_name`?
        }

        UnpackedKPattern {
            kpuzzle: self.kpuzzle.clone(),
            kpattern_data: pattern_data.into(),
        }
    }

    pub fn apply_alg(&self, alg: &Alg) -> Result<UnpackedKPattern, UnpackedInvalidAlgError> {
        let transformation = self.kpuzzle.transformation_from_alg(alg)?;
        Ok(self.apply_transformation(&transformation))
    }

    pub fn apply_move(&self, m: &Move) -> Result<UnpackedKPattern, UnpackedInvalidAlgError> {
        let transformation = self.kpuzzle.transformation_from_move(m)?;
        Ok(self.apply_transformation(&transformation))
    }
}

impl PartialEq<UnpackedKPattern> for UnpackedKPattern {
    fn eq(&self, other: &Self) -> bool {
        // TODO: check if the KPuzzle comparison is correct and performant.
        self.kpuzzle == other.kpuzzle && self.kpattern_data == other.kpattern_data
    }
}
