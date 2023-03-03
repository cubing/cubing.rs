use std::{collections::HashMap, rc::Rc};

use super::KPuzzleDefinition;

#[derive(Debug, Clone)]
pub struct KTransformation {
    // TODO: store the orbits directly?
    pub definition: Rc<KPuzzleDefinition>,
    pub transformation_data: Rc<KTransformationData>, // TODO: check that this is immutable
}
pub type KTransformationData = HashMap<String, KTransformationOrbitData>;
#[derive(Debug, Clone)]
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
            for i in 0..(num_pieces - 1) {
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
}
