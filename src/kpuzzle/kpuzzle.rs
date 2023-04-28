use std::{collections::HashMap, rc::Rc};

use crate::alg::{Alg, AlgNode, Move};

use super::{
    KPuzzleDefinition, KState, KTransformation, KTransformationData, KTransformationOrbitData,
};

#[derive(Debug)]
pub struct KPuzzleData {
    pub definition: Rc<KPuzzleDefinition>,

    // TODO: compute lazily while being thread-safe?
    cached_identity_transformation_data: Rc<KTransformationData>,
}

#[derive(Debug, Clone)]
pub struct KPuzzle {
    data: Rc<KPuzzleData>,
}

// TODO: Get rid of this in favor of purely `KTransformation` and `KState`?
impl KPuzzle {
    pub fn new(definition: impl Into<Rc<KPuzzleDefinition>>) -> Self {
        let definition = definition.into();
        let cached_identity_transformation_data = identity_transformation_data(&definition).into();
        let data = KPuzzleData {
            definition,
            cached_identity_transformation_data,
        }
        .into();
        KPuzzle { data }
    }

    pub fn definition(&self) -> &KPuzzleDefinition {
        &self.data.as_ref().definition
    }

    // TODO: implement this as a `TryFrom`?
    pub fn transformation_from_move(
        &self, // TODO: Any issues with not using `&self`?
        r#move: &Move,
    ) -> Result<KTransformation, String> {
        let q = r#move.quantum.to_string();
        let transformation_data = self
            .data
            .definition
            .moves
            .get(&q)
            .ok_or_else(|| format!("Unknown move quantum: {}", q))?
            .clone();
        Ok(KTransformation {
            kpuzzle: self.clone(),
            transformation_data,
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
        KTransformation {
            kpuzzle: self.clone(),
            transformation_data: self.data.cached_identity_transformation_data.clone(),
        }
    }

    pub fn start_state(&self) -> KState {
        let state_data = self.data.definition.start_state_data.clone();
        KState {
            kpuzzle: self.clone(),
            state_data,
        }
    }
}

impl PartialEq<KPuzzle> for KPuzzle {
    fn eq(&self, other: &Self) -> bool {
        // TODO: is this ref comparison safe?
        std::ptr::eq(self.data.as_ref(), other.data.as_ref())
    }
}

impl From<KPuzzleDefinition> for KPuzzle {
    fn from(input: KPuzzleDefinition) -> KPuzzle {
        KPuzzle::new(input)
    }
}

fn identity_transformation_data(definition: &KPuzzleDefinition) -> KTransformationData {
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
    transformation_data
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
        AlgNode::PauseNode(_pause) => Ok(kpuzzle.identity_transformation()),
        AlgNode::NewlineNode(_pause) => Ok(kpuzzle.identity_transformation()),
        AlgNode::LineCommentNode(_pause) => Ok(kpuzzle.identity_transformation()),
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
