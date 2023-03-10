use std::{collections::HashMap, rc::Rc};

use crate::alg::{Alg, Amount};

use super::{identity_transformation, KPuzzle, KPuzzleDefinition};

#[derive(Debug, Clone)]
pub struct KTransformation {
    // TODO: store the orbits directly?
    pub definition: Rc<KPuzzleDefinition>,
    pub transformation_data: Rc<KTransformationData>, // TODO: check that this is immutable
}
// TODO: Use `Move` as the key?
pub type KTransformationData = HashMap<String, KTransformationOrbitData>;

impl PartialEq<KTransformation> for KTransformation {
    fn eq(&self, other: &Self) -> bool {
        // TODO: is this ref comparison safe?
        std::ptr::eq(self.definition.as_ref(), other.definition.as_ref())
            && self.transformation_data == other.transformation_data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KTransformationOrbitData {
    pub permutation: Vec<usize>,
    pub orientation: Vec<usize>,
}

impl KTransformation {
    pub fn apply_transformation(&self, other: &Self) -> Self {
        let mut transformation_data: KTransformationData = HashMap::new();
        for (orbit_name, orbit_definition) in &self.definition.orbits {
            let num_pieces = orbit_definition.num_pieces;

            let mut permutation = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?
            let mut orientation = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?

            let self_orbit = &self.transformation_data[orbit_name];
            let other_orbit = &other.transformation_data[orbit_name];

            // TODO: optimization when either value is the identity.
            for i in 0..num_pieces {
                permutation[i] = self_orbit.permutation[other_orbit.permutation[i]];
                orientation[i] = (self_orbit.orientation[other_orbit.permutation[i]]
                    + other_orbit.orientation[i])
                    % orbit_definition.num_orientations;
            }

            let orbit_data = KTransformationOrbitData {
                permutation,
                orientation,
            };
            transformation_data.insert(orbit_name.into(), orbit_data); // TODO: why do we need to coerce `orbit_name`?
        }
        KTransformation {
            definition: self.definition.clone(),
            transformation_data: Rc::new(transformation_data),
        }
    }

    pub fn invert(&self) -> Self {
        let mut transformation_data: KTransformationData = HashMap::new();
        for (orbit_name, orbit_definition) in &self.definition.orbits {
            let num_pieces = orbit_definition.num_pieces;

            let mut permutation = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?
            let mut orientation = vec![0; num_pieces]; // TODO: can we safely avoid initializing the entries?

            let self_orbit = &self.transformation_data[orbit_name];

            // TODO: optimization when either value is the identity.
            for i in 0..num_pieces {
                let from_idx = self_orbit.permutation[i];
                permutation[from_idx] = i;
                orientation[from_idx] = (orbit_definition.num_orientations
                    - self_orbit.orientation[i])
                    .rem_euclid(orbit_definition.num_orientations)
            }

            let orbit_data = KTransformationOrbitData {
                permutation,
                orientation,
            };
            transformation_data.insert(orbit_name.into(), orbit_data); // TODO: why do we need to coerce `orbit_name`?
        }
        KTransformation {
            definition: self.definition.clone(),
            transformation_data: Rc::new(transformation_data),
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
            return identity_transformation(&self.definition);
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
    type Error = String;

    fn try_from(input: (&KPuzzle, &Alg)) -> Result<Self, Self::Error> {
        let (kpuzzle, alg) = input;
        kpuzzle.transformation_from_alg(alg)
    }
}

impl TryFrom<(&KPuzzle, &str)> for KTransformation {
    type Error = String;

    fn try_from(input: (&KPuzzle, &str)) -> Result<Self, Self::Error> {
        let (kpuzzle, s) = input;
        KTransformation::try_from((kpuzzle, &s.parse::<Alg>()?))
    }
}
