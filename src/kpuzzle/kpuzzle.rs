use std::{collections::HashMap, rc::Rc};

use crate::alg::{Alg, AlgNode};

use super::{KPuzzleDefinition, KTransformation, KTransformationData, KTransformationOrbitData};

#[derive(Debug)]
pub struct KPuzzle {
    pub definition: Rc<KPuzzleDefinition>,

    // TODO: compute lazily while being thread-safe?
    cached_identity_transformation: KTransformation,
}

// TODO: Get rid of this in favor of purely `KTransformation` and `KState`?
impl KPuzzle {
    pub fn new(definition: impl Into<Rc<KPuzzleDefinition>>) -> Self {
        let definition = definition.into();
        let cached_identity_transformation = identity_transformation(&definition);
        KPuzzle {
            definition,
            cached_identity_transformation,
        }
    }

    // TODO: implement this as a `TryFrom`?
    pub fn transformation_from_move(
        &self, // TODO: Any issues with not using `&self`?
        r#move: &crate::alg::Move,
    ) -> Result<KTransformation, String> {
        let s = r#move.quantum.to_string();
        let data = self.definition.moves.get(&s).ok_or("Unknown move name.")?;
        Ok(KTransformation {
            definition: self.definition.clone(),
            transformation_data: data.clone(),
        }
        .self_multiply(r#move.amount))
    }

    // TODO: implement this as a `TryFrom`?
    pub fn transformation_from_alg(
        &self, // TODO: Any issues with not using `&self`?
        alg: &crate::alg::Alg,
    ) -> Result<KTransformation, String> {
        transformation_from_alg(self, alg)
    }

    // TODO: implement this as a `TryFrom`?
    pub fn transformation_from_str(
        &self, // TODO: Any issues with not using `&self`?
        alg_str: &str,
    ) -> Result<KTransformation, String> {
        transformation_from_alg(self, &alg_str.parse::<Alg>()?)
    }

    pub fn identity_transformation(&self) -> KTransformation {
        self.cached_identity_transformation.clone()
    }
}

impl From<KPuzzleDefinition> for KPuzzle {
    fn from(input: KPuzzleDefinition) -> KPuzzle {
        KPuzzle::new(input)
    }
}
pub fn identity_transformation(definition: &Rc<KPuzzleDefinition>) -> KTransformation {
    let mut transformation_data: KTransformationData = HashMap::new();
    for (orbit_name, orbit_definition) in &definition.orbits {
        let num_pieces = orbit_definition.num_pieces;

        let permutation = (0..num_pieces).collect();
        let orientation = vec![0; num_pieces];

        let orbit_data = KTransformationOrbitData {
            permutation,
            orientation,
        };
        transformation_data.insert(orbit_name.into(), orbit_data);
    }
    KTransformation {
        definition: definition.clone(),
        transformation_data: Rc::new(transformation_data),
    }
}

fn transformation_from_alg(kpuzzle: &KPuzzle, alg: &Alg) -> Result<KTransformation, String> {
    let mut t = kpuzzle.identity_transformation();
    for node in alg.nodes.iter() {
        let node_transformation = transformation_from_alg_node(kpuzzle, node)?;
        t = t.apply_transformation(&node_transformation);
    }
    Ok(t)
}

fn transformation_from_alg_node(
    kpuzzle: &KPuzzle,
    alg_node: &AlgNode,
) -> Result<KTransformation, String> {
    match alg_node {
        AlgNode::MoveNode(r#move) => kpuzzle.transformation_from_move(r#move),
        AlgNode::GroupingNode(grouping) => {
            Ok(transformation_from_alg(kpuzzle, &grouping.alg)?.self_multiply(grouping.amount))
        }
        AlgNode::CommutatorNode(commutator) => {
            let a = transformation_from_alg(kpuzzle, &commutator.a)?;
            let b = transformation_from_alg(kpuzzle, &commutator.b)?;
            let a_prime = transformation_from_alg(kpuzzle, &commutator.a.invert())?; // TODO: invert the transformation instead of the alg!
            let b_prime = transformation_from_alg(kpuzzle, &commutator.a.invert())?; // TODO: invert the transformation instead of the alg!
            Ok(a.apply_transformation(&b)
                .apply_transformation(&a_prime)
                .apply_transformation(&b_prime))
        }
        AlgNode::ConjugateNode(conjugate) => {
            let a = transformation_from_alg(kpuzzle, &conjugate.a)?;
            let b = transformation_from_alg(kpuzzle, &conjugate.b)?;
            let a_prime = transformation_from_alg(kpuzzle, &conjugate.a.invert())?; // TODO: invert the transformation instead of the alg!
            Ok(a.apply_transformation(&b).apply_transformation(&a_prime))
        }
    }
}
