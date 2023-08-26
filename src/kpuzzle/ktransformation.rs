use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::alg::{Alg, Amount};

use super::{InvalidAlgError, KPuzzle, KPuzzleOrbitName};

#[derive(Debug, Clone)]
pub struct KTransformation {
    // TODO: store the orbits directly?
    pub kpuzzle: KPuzzle,
    pub ktransformation_data: Arc<KTransformationData>, // TODO: check that this is immutable
}
// TODO: Use `Move` as the key?
pub type KTransformationData = HashMap<KPuzzleOrbitName, KTransformationOrbitData>;

impl PartialEq<KTransformation> for KTransformation {
    fn eq(&self, other: &Self) -> bool {
        // TODO: check if the KPuzzle comparison is correct and performant.
        self.kpuzzle == other.kpuzzle && self.ktransformation_data == other.ktransformation_data
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KTransformationOrbitData {
    pub permutation: Vec<usize>,
    pub orientation_delta: Vec<usize>,
}

impl KTransformation {
    pub fn apply_transformation(&self, other: &Self) -> Self {
        let mut transformation_data: KTransformationData = HashMap::new();
        for orbit_definition in &self.kpuzzle.definition().orbits {
            let num_pieces = orbit_definition.num_pieces;

            let mut permutation = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?
            let mut orientation_delta = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?

            let self_orbit = &self.ktransformation_data[&orbit_definition.orbit_name];
            let other_orbit = &other.ktransformation_data[&orbit_definition.orbit_name];

            // TODO: optimization when either value is the identity.
            for i in 0..num_pieces {
                permutation[i] = self_orbit.permutation[other_orbit.permutation[i]];
                orientation_delta[i] = (self_orbit.orientation_delta[other_orbit.permutation[i]]
                    + other_orbit.orientation_delta[i])
                    % orbit_definition.num_orientations;
            }

            let orbit_data = KTransformationOrbitData {
                permutation,
                orientation_delta,
            };
            transformation_data.insert(orbit_definition.orbit_name.clone(), orbit_data);
            // TODO: why do we need to coerce `orbit_name`?
        }
        KTransformation {
            kpuzzle: self.kpuzzle.clone(),
            ktransformation_data: Arc::new(transformation_data),
        }
    }

    pub fn invert(&self) -> Self {
        let mut transformation_data: KTransformationData = HashMap::new();
        for orbit_definition in &self.kpuzzle.definition().orbits {
            let num_pieces = orbit_definition.num_pieces;

            let mut permutation = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?
            let mut orientation_delta = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?

            let self_orbit = &self.ktransformation_data[&orbit_definition.orbit_name];

            // TODO: optimization when either value is the identity.
            for i in 0..num_pieces {
                let from_idx = self_orbit.permutation[i];
                permutation[from_idx] = i;
                orientation_delta[from_idx] = (orbit_definition.num_orientations
                    - self_orbit.orientation_delta[i])
                    .rem_euclid(orbit_definition.num_orientations)
            }

            let orbit_data = KTransformationOrbitData {
                permutation,
                orientation_delta,
            };
            transformation_data.insert(orbit_definition.orbit_name.clone(), orbit_data);
            // TODO: why do we need to coerce `orbit_name`?
        }
        KTransformation {
            kpuzzle: self.kpuzzle.clone(),
            ktransformation_data: Arc::new(transformation_data),
        }
    }

    pub fn self_multiply(&self, amount: Amount) -> Self {
        if amount == 1 {
            return self.clone();
        }
        if amount < 0 {
            return self.invert().self_multiply(-amount);
        }
        if amount == 0 {
            // TODO: use cached identity transformations from `KPuzzle`???
            return self.kpuzzle.identity_transformation();
        }
        let twice_halfish = if amount == 2 {
            // We'd share this `apply_transformation` with the other branch, but that triggers a bug in the borrow checker(!)
            // https://github.com/rust-lang/rust/issues/54663
            self.apply_transformation(self)
        } else {
            println!("--{}--", amount / 2);
            let halfish = self.self_multiply(amount / 2);
            halfish.apply_transformation(&halfish)
        };
        if amount % 2 == 0 {
            twice_halfish
        } else {
            self.apply_transformation(&twice_halfish)
        }
    }
}

impl TryFrom<(&KPuzzle, &Alg)> for KTransformation {
    type Error = InvalidAlgError;

    fn try_from(input: (&KPuzzle, &Alg)) -> Result<Self, Self::Error> {
        let (kpuzzle, alg) = input;
        kpuzzle.transformation_from_alg(alg)
    }
}

impl TryFrom<(&KPuzzle, &str)> for KTransformation {
    type Error = InvalidAlgError;

    fn try_from(input: (&KPuzzle, &str)) -> Result<Self, Self::Error> {
        let (kpuzzle, s) = input;
        KTransformation::try_from((kpuzzle, &s.parse::<Alg>()?))
    }
}

impl TryFrom<(KPuzzle, &str)> for KTransformation {
    type Error = InvalidAlgError;

    fn try_from(input: (KPuzzle, &str)) -> Result<Self, Self::Error> {
        let (kpuzzle, s) = input;
        KTransformation::try_from((&kpuzzle, &s.parse::<Alg>()?))
    }
}
