use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::alg::{Alg, AlgNode, AlgParseError, Move};

use super::{
    KPuzzleDefinition, KState, KTransformation, KTransformationData, KTransformationOrbitData,
};

/// An error due to the structure of a [`KPuzzleDefinition`] (such as a recursive derived move definition).
#[derive(Debug)]
pub struct InvalidDefinitionError {
    pub description: String,
}

// TODO: is Rust smart enough to optimize this using just the `From<&str>` definition?
impl From<String> for InvalidDefinitionError {
    fn from(description: String) -> Self {
        Self { description }
    }
}

impl From<&str> for InvalidDefinitionError {
    fn from(description: &str) -> Self {
        Self {
            description: description.to_owned(),
        }
    }
}

#[derive(Debug)]
// An operation failed due to an invalid error. This usually occurs when applying an alg to a puzzle with incompatible notation.
pub struct InvalidMoveError {
    pub description: String,
}

// TODO: is Rust smart enough to optimize this using just the `From<&str>` definition?
impl From<String> for InvalidMoveError {
    fn from(description: String) -> Self {
        Self { description }
    }
}

impl From<&str> for InvalidMoveError {
    fn from(description: &str) -> Self {
        Self {
            description: description.to_owned(),
        }
    }
}

/// An error type that can indicate multiple error causes, when parsing and applying an alg at the same time.
#[derive(derive_more::From, Debug)]
pub enum InvalidAlgError {
    AlgParse(AlgParseError),
    InvalidMove(InvalidMoveError),
}

#[derive(Debug)]
pub struct KPuzzleData {
    pub definition: Arc<KPuzzleDefinition>,

    // TODO: compute lazily while being thread-safe?
    cached_identity_transformation_data: Arc<KTransformationData>,
}

enum DerivedMoveVisitStatus {
    InProgress(()),
    Done(()),
}
// TODO: handle move definitions like x2.
struct DerivedMovesValidator<'a> {
    def: &'a KPuzzleDefinition,
    derived_move_visit_statuses: HashMap<Move, DerivedMoveVisitStatus>,
}

impl DerivedMovesValidator<'_> {
    pub fn check(def: &KPuzzleDefinition) -> Result<(), InvalidDefinitionError> {
        if let Some(derived_moves) = &def.experimental_derived_moves {
            let mut validator = DerivedMovesValidator {
                def,
                derived_move_visit_statuses: HashMap::default(),
            };
            for (derived_move, _) in derived_moves.iter() {
                validator.visit(derived_move)?
            }
        }
        Ok(())
    }

    fn visit(&mut self, key_move: &Move) -> Result<(), InvalidDefinitionError> {
        match self.derived_move_visit_statuses.get(key_move) {
            Some(DerivedMoveVisitStatus::InProgress(())) => {
                return Err(format!("Recursive derived move definition for: {}", key_move).into());
            }
            Some(DerivedMoveVisitStatus::Done(())) => return Ok(()),
            None => (),
        };
        self.derived_move_visit_statuses.insert(
            key_move.clone(), /* Avoid this clonse by using lifetimes? */
            DerivedMoveVisitStatus::InProgress(()),
        );
        let move_lookup_result = match lookup_move(self.def, key_move) {
            Some(move_lookup_result) => move_lookup_result,
            None => {
                return Err("Invalid move??".into());
            }
        };
        match move_lookup_result.source {
            MoveLookupResultSource::DirectlyDefined(_) => {}
            MoveLookupResultSource::DerivedFromAlg(alg) => {
                let descendant_move_keys = self.ancestor_move_keys_in_alg(alg)?;
                for descendant_move_key in descendant_move_keys {
                    self.visit(&descendant_move_key)?
                }
            }
        };
        // TODO: Would it help to save `descendant_move_keys` for something?
        self.derived_move_visit_statuses.insert(
            key_move.clone(), /* Avoid this clonse by using lifetimes? */
            DerivedMoveVisitStatus::Done(()),
        );
        Ok(())
    }

    fn ancestor_move_keys_in_alg(&self, alg: &Alg) -> Result<HashSet<Move>, String> {
        let mut descendant_move_keys = HashSet::<Move>::default();
        for node in &alg.nodes {
            self.ancestor_move_keys_in_node(node, &mut descendant_move_keys)?
        }
        Ok(descendant_move_keys)
    }

    fn ancestor_move_keys_in_alg_recursive(
        &self,
        alg: &Alg,
        descendant_move_keys: &mut HashSet<Move>, // TODO: figure out how to avoid owning `Move`s?
    ) -> Result<(), String> {
        for node in &alg.nodes {
            self.ancestor_move_keys_in_node(node, descendant_move_keys)?
        }
        Ok(())
    }

    fn ancestor_move_keys_in_node<'a, 'b: 'a>(
        &self,
        node: &'a AlgNode,
        descendant_move_keys: &mut HashSet<Move>,
    ) -> Result<(), String> {
        match node {
            AlgNode::GroupingNode(grouping) => {
                self.ancestor_move_keys_in_alg_recursive(&grouping.alg, descendant_move_keys)?
            }
            AlgNode::CommutatorNode(commutator) => {
                self.ancestor_move_keys_in_alg_recursive(&commutator.a, descendant_move_keys)?;

                self.ancestor_move_keys_in_alg_recursive(&commutator.b, descendant_move_keys)?
            }
            AlgNode::ConjugateNode(conjugate) => {
                self.ancestor_move_keys_in_alg_recursive(&conjugate.a, descendant_move_keys)?;

                self.ancestor_move_keys_in_alg_recursive(&conjugate.b, descendant_move_keys)?
            }
            AlgNode::MoveNode(key_move) => {
                let move_lookup_result = match lookup_move(self.def, key_move) {
                    Some(move_lookup_result) => move_lookup_result,
                    None => {
                        return Err(format!(
                            "Invalid move used in a derived move definition: {}",
                            key_move
                        ))
                    }
                };
                descendant_move_keys.insert(move_lookup_result.key_move.clone());
                // TODO: figure out how to avoid the need to clone.
            }
            _ => (),
        };
        Ok(())
    }
}

fn move_with_amount_1(r#move: &Move) -> Move {
    Move {
        quantum: r#move.quantum.clone(),
        amount: 1,
    }
}

// Based on: https://github.com/cubing/cubing.js/blob/5eac388a09f6cf102fdf6d6a1cbb5d20a667ddfd/src/cubing/kpuzzle/construct.ts#L57-L101
fn lookup_move<'a>(def: &'a KPuzzleDefinition, r#move: &Move) -> Option<MoveLookupResult<'a>> {
    // TODO: support looking up moves directly by quantum.

    // Handle cases by order of commonality.
    if let Some((key_move, source)) = def.moves.get_key_value(&move_with_amount_1(r#move)) {
        return Some(MoveLookupResult {
            key_move,
            relative_amount: r#move.amount,
            source: MoveLookupResultSource::DirectlyDefined(source),
        });
    };
    if let Some(derived_moves) = &def.experimental_derived_moves {
        if let Some((key_move, source)) = derived_moves.get_key_value(r#move) {
            return Some(MoveLookupResult {
                key_move,
                relative_amount: r#move.amount,
                source: MoveLookupResultSource::DerivedFromAlg(source),
            });
        };
    }
    // Exact match (e.g. y2 on clock)
    if let Some((key_move, source)) = def.moves.get_key_value(&r#move_with_amount_1(r#move)) {
        return Some(MoveLookupResult {
            key_move,
            relative_amount: 1,
            source: MoveLookupResultSource::DirectlyDefined(source),
        });
    };
    if let Some(derived_moves) = &def.experimental_derived_moves {
        if let Some((key_move, source)) = derived_moves.get_key_value(r#move) {
            return Some(MoveLookupResult {
                key_move,
                relative_amount: 1,
                source: MoveLookupResultSource::DerivedFromAlg(source),
            });
        };
    }
    // Inverse match (e.g. y2' on clock)
    if let Some((key_move, source)) = def.moves.get_key_value(&r#move.invert()) {
        return Some(MoveLookupResult {
            key_move,
            relative_amount: -1,
            source: MoveLookupResultSource::DirectlyDefined(source),
        });
    };
    if let Some(derived_moves) = &def.experimental_derived_moves {
        if let Some((key_move, source)) = derived_moves.get_key_value(&r#move.invert()) {
            return Some(MoveLookupResult {
                key_move,
                relative_amount: -1,
                source: MoveLookupResultSource::DerivedFromAlg(source),
            });
        };
    }
    None
}

#[derive(Debug, Clone)]
pub struct KPuzzle {
    data: Arc<KPuzzleData>,
}

enum MoveLookupResultSource<'a> {
    DirectlyDefined(&'a Arc<KTransformationData>),
    DerivedFromAlg(&'a Alg), // TODO: parse and store these algs at `KPuzzle` instantiation time.
}

struct MoveLookupResult<'a> {
    key_move: &'a Move,
    relative_amount: i32,
    source: MoveLookupResultSource<'a>,
}

// TODO: Get rid of this in favor of purely `KTransformation` and `KState`?
impl KPuzzle {
    pub fn try_new(
        definition: impl Into<Arc<KPuzzleDefinition>>,
    ) -> Result<Self, InvalidDefinitionError> {
        let definition = definition.into();
        let cached_identity_transformation_data = identity_transformation_data(&definition).into();
        let data = KPuzzleData {
            definition,
            cached_identity_transformation_data,
        }
        .into();
        let kpuzzle = KPuzzle { data };
        DerivedMovesValidator::check(&kpuzzle.data.definition)?;
        Ok(kpuzzle)
    }

    pub fn definition(&self) -> &KPuzzleDefinition {
        &self.data.definition
    }

    // TODO: implement this as a `TryFrom`?
    pub fn transformation_from_move(
        &self, // TODO: Any issues with not using `&self`?
        key_move: &Move,
    ) -> Result<KTransformation, InvalidAlgError> {
        let move_lookup_result = match lookup_move(self.definition(), key_move) {
            Some(move_lookup_result) => move_lookup_result,
            None => {
                return Err(InvalidMoveError {
                    description: format!("Move does not exist on this puzzle: {}", key_move),
                }
                .into())
            }
        };
        let transformation = match move_lookup_result.source {
            // TODO: Avoid constructing this `KTransformation`.
            MoveLookupResultSource::DirectlyDefined(transformation_data) => KTransformation {
                kpuzzle: self.clone(),
                transformation_data: transformation_data.clone(),
            },
            MoveLookupResultSource::DerivedFromAlg(alg) => self.transformation_from_alg(alg)?,
        };
        Ok(transformation.self_multiply(move_lookup_result.relative_amount))
    }

    // TODO: implement this as a `TryFrom`?
    pub fn transformation_from_alg(
        &self, // TODO: Any issues with not using `&self`?
        alg: &crate::alg::Alg,
    ) -> Result<KTransformation, InvalidAlgError> {
        transformation_from_alg(self, alg)
    }

    // TODO: implement this as a `TryFrom`?
    pub fn transformation_from_str(
        &self, // TODO: Any issues with not using `&self`?
        alg_str: &str,
    ) -> Result<KTransformation, InvalidAlgError> {
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

impl TryFrom<KPuzzleDefinition> for KPuzzle {
    type Error = InvalidDefinitionError;
    fn try_from(input: KPuzzleDefinition) -> Result<KPuzzle, InvalidDefinitionError> {
        KPuzzle::try_new(input)
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
        transformation_data.insert(orbit_name.clone(), orbit_data);
    }
    transformation_data
}

fn transformation_from_alg(
    kpuzzle: &KPuzzle,
    alg: &Alg,
) -> Result<KTransformation, InvalidAlgError> {
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
) -> Result<KTransformation, InvalidAlgError> {
    match alg_node {
        AlgNode::MoveNode(key_move) => kpuzzle.transformation_from_move(key_move),
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
