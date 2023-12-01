use crate::alg::{Alg, Move};

use super::super::{KPuzzleDefinition, KTransformationData};

pub(crate) enum MoveLookupResultSource<'a> {
    DirectlyDefined(&'a KTransformationData),
    DerivedFromAlg(&'a Alg), // TODO: parse and store these algs at `KPuzzle` instantiation time.
}

pub(crate) struct MoveLookupResult<'a> {
    pub(crate) key_move: &'a Move,
    pub(crate) relative_amount: i32,
    pub(crate) source: MoveLookupResultSource<'a>,
}

fn move_with_amount_1(r#move: &Move) -> Move {
    Move {
        quantum: r#move.quantum.clone(),
        amount: 1,
    }
}
// Based on: https://github.com/cubing/cubing.js/blob/5eac388a09f6cf102fdf6d6a1cbb5d20a667ddfd/src/cubing/kpuzzle/construct.ts#L57-L101
pub(crate) fn lookup_move<'a>(
    def: &'a KPuzzleDefinition,
    r#move: &Move,
) -> Option<MoveLookupResult<'a>> {
    // TODO: support looking up moves directly by quantum.

    // Handle cases by order of commonality.
    if let Some((key_move, source)) = def.moves.get_key_value(&move_with_amount_1(r#move)) {
        return Some(MoveLookupResult {
            key_move,
            relative_amount: r#move.amount,
            source: MoveLookupResultSource::DirectlyDefined(source),
        });
    };
    if let Some(derived_moves) = &def.derived_moves {
        if let Some((key_move, source)) = derived_moves.get_key_value(&move_with_amount_1(r#move)) {
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
    if let Some(derived_moves) = &def.derived_moves {
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
    if let Some(derived_moves) = &def.derived_moves {
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
