use std::{
    alloc::Layout,
    error::Error,
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::{
    alg::{Alg, AlgNode, AlgParseError, Move},
    kpuzzle::{KPuzzleDefinition, KPuzzleOrbitName},
};

use super::{
    byte_conversions::usize_to_u8,
    derived_moves_validator::DerivedMovesValidator,
    lookup_move::{lookup_move, MoveLookupResultSource},
    orientation_packer::OrientationPacker,
    packed_orbit_data::PackedOrbitData,
    InvalidPatternDataError, KPattern, KTransformation,
};

// TODO: allow certain values over 107?
const MAX_NUM_ORIENTATIONS_INCLUSIVE: usize = 107;

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

impl Display for InvalidDefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
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

impl Display for InvalidMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

/// An error type that can indicate multiple error causes, when parsing and applying an alg at the same time.
#[derive(derive_more::From, Debug, derive_more::Display)]
pub enum InvalidAlgError {
    AlgParse(AlgParseError),
    InvalidMove(InvalidMoveError),
}

impl Error for InvalidAlgError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

fn identity_transformation(kpuzzle: &KPuzzle) -> KTransformation {
    let mut packed_orbit_data = PackedOrbitData::new_with_uninitialized_bytes(kpuzzle.clone());
    for orbit_info in &kpuzzle.data.orbit_iteration_info {
        // for orbit_definition in &kpuzzle.definition.orbits {
        let num_pieces = orbit_info.num_pieces;
        for i in 0..num_pieces {
            packed_orbit_data.set_raw_piece_or_permutation_value(orbit_info, i, i as u8);
            packed_orbit_data.set_raw_orientation_value(orbit_info, i, 0);
        }
    }
    KTransformation { packed_orbit_data }
}

#[derive(Debug)]
pub struct KPuzzleOrbitInfo {
    pub name: KPuzzleOrbitName,
    pub pieces_or_permutations_offset: usize,
    pub orientations_offset: usize,
    pub num_pieces: usize,
    pub num_orientations: u8,
    pub orientation_packer: OrientationPacker,
}

#[derive(Debug)]
pub struct KPuzzleData {
    pub definition: Arc<KPuzzleDefinition>,
    // TODO: compute lazily while being thread-safe?
    // cached_identity_transformation_data: PackedOrbitData, // TODO

    // Private cached values.
    pub num_bytes: usize,
    pub orbit_iteration_info: Vec<KPuzzleOrbitInfo>,
    pub layout: Layout,
}

#[derive(Clone)]
pub struct KPuzzle {
    pub data: Arc<KPuzzleData>, // TODO
                                // pub data: PackedKPuzzleData,
}

/// An error type that can indicate multiple error causes, when parsing and applying an alg at the same time.
#[derive(derive_more::From, Debug, derive_more::Display)]
pub enum ConversionError {
    InvalidAlg(InvalidAlgError),
    InvalidDefinition(InvalidDefinitionError),
    InvalidPatternData(InvalidPatternDataError),
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
            let b_prime = transformation_from_alg(kpuzzle, &commutator.b.invert())?; // TODO: invert the transformation instead of the alg!
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

impl KPuzzle {
    pub fn try_new(
        definition: impl Into<Arc<KPuzzleDefinition>>,
    ) -> Result<Self, InvalidDefinitionError> {
        let definition = definition.into();
        // let cached_identity_transformation_data = identity_transformation_data(&definition).into(); // TODO

        DerivedMovesValidator::check(&definition)?;

        let mut bytes_offset = 0;
        let mut orbit_iteration_info: Vec<KPuzzleOrbitInfo> = vec![];

        for orbit_definition in &definition.orbits {
            let num_orientations = orbit_definition.num_orientations;
            if num_orientations > MAX_NUM_ORIENTATIONS_INCLUSIVE {
                return Err(InvalidDefinitionError { description: format!("`num_orientations` for orbit {} is too large ({}). Maximum is {} for the current build." , orbit_definition.orbit_name, num_orientations, MAX_NUM_ORIENTATIONS_INCLUSIVE)});
            }
            orbit_iteration_info.push({
                KPuzzleOrbitInfo {
                    name: orbit_definition.orbit_name.clone(),
                    num_pieces: orbit_definition.num_pieces,
                    num_orientations: usize_to_u8(num_orientations),
                    pieces_or_permutations_offset: bytes_offset,
                    orientations_offset: bytes_offset + orbit_definition.num_pieces,
                    orientation_packer: OrientationPacker::new(orbit_definition.num_orientations),
                }
            });
            bytes_offset += orbit_definition.num_pieces * 2;
        }

        Ok(Self {
            data: Arc::new(KPuzzleData {
                definition,
                num_bytes: bytes_offset,
                orbit_iteration_info,
                layout: Layout::array::<u8>(bytes_offset).map_err(|_| InvalidDefinitionError {
                    description: "Could not construct packed layout.".to_owned(),
                })?,
            }),
        })
    }

    pub fn try_from_json(json_bytes: &[u8]) -> Result<KPuzzle, InvalidDefinitionError> {
        // TODO: implement this directly
        let definition: KPuzzleDefinition = match serde_json::from_slice(json_bytes) {
            Ok(kpuzzle_data) => kpuzzle_data,
            Err(e) => {
                return Err(InvalidDefinitionError {
                    description: e.to_string().to_owned(),
                })
            }
        };
        KPuzzle::try_new(definition)
    }

    pub fn definition(&self) -> &KPuzzleDefinition {
        &self.data.definition
    }

    pub fn default_pattern(&self) -> KPattern {
        // TODO: check/cache at construction time.
        KPattern::try_from_data(self, &self.data.definition.default_pattern)
            .expect("Invalid default pattern")
    }

    // TODO: design a much much more efficient API.
    pub fn lookup_orbit_info(&self, orbit_name: &KPuzzleOrbitName) -> Option<&KPuzzleOrbitInfo> {
        self.data
            .orbit_iteration_info
            .iter()
            .find(|&orbit_info| &orbit_info.name == orbit_name)
    }

    pub fn identity_transformation(&self) -> KTransformation {
        identity_transformation(self)
    }

    // TODO: implement this as a `TryFrom`?
    pub fn transformation_from_move(
        &self, // TODO: Any issues with not using `&self`?
        key_move: &Move,
    ) -> Result<KTransformation, InvalidAlgError> {
        let move_lookup_result = match lookup_move(&self.data.definition, key_move) {
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
            MoveLookupResultSource::DirectlyDefined(transformation_data) => {
                KTransformation::try_from_data(self, transformation_data)
                    .expect("TODO: invalid definition — this should be caught earlier")
            }
            MoveLookupResultSource::DerivedFromAlg(alg) => self.transformation_from_alg(alg)?,
        };
        Ok(transformation.self_multiply(move_lookup_result.relative_amount))
    }

    // TODO: implement this directly
    pub fn transformation_from_alg(&self, alg: &Alg) -> Result<KTransformation, InvalidAlgError> {
        transformation_from_alg(self, alg)
    }
}

impl TryFrom<KPuzzleDefinition> for KPuzzle {
    type Error = InvalidDefinitionError;

    fn try_from(definition: KPuzzleDefinition) -> Result<Self, Self::Error> {
        KPuzzle::try_new(definition)
    }
}

impl Debug for KPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ … name: \"{}\" … }}", &self.data.definition.name)
    }
}
