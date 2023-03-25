use cubing::kpuzzle::KState;

use crate::triggers::F2LSlot;

#[derive(Clone, Default)]
#[allow(non_snake_case)]
pub struct SlotMask {
    pub H: bool,
    pub I: bool,
    pub J: bool,
    pub K: bool,
}

impl SlotMask {
    pub fn is_solved(&self, f2l_slot: &F2LSlot) -> bool {
        match f2l_slot {
            F2LSlot::H => self.H,
            F2LSlot::I => self.I,
            F2LSlot::J => self.J,
            F2LSlot::K => self.K,
        }
    }
    pub fn set(&self, f2l_slot: &F2LSlot, solved: bool) -> SlotMask {
        let mut new_mask = self.clone();
        match f2l_slot {
            F2LSlot::H => new_mask.H = solved,
            F2LSlot::I => new_mask.I = solved,
            F2LSlot::J => new_mask.J = solved,
            F2LSlot::K => new_mask.K = solved,
        };
        new_mask
    }
}

// TODO: is it more efficient not to borrow `F2LSlot`?
pub fn is_slot_solved(state: &KState, f2l_slot: &F2LSlot) -> bool {
    // Reid order:
    // UF  UR  UB  UL  . DF  DR  DB  DL  . FR  FL  BR  BL
    // UFR URB UBL ULF . DRF DFL DLB DBR
    // U L F R B D
    match f2l_slot {
        F2LSlot::H => are_slot_pieces_solved(state, 9, 5),
        F2LSlot::I => are_slot_pieces_solved(state, 11, 6),
        F2LSlot::J => are_slot_pieces_solved(state, 10, 7),
        F2LSlot::K => are_slot_pieces_solved(state, 8, 4),
    }
}

pub fn are_slot_pieces_solved(state: &KState, edge_idx: usize, corner_idx: usize) -> bool {
    is_piece_solved(state, "EDGES", edge_idx) && is_piece_solved(state, "CORNERS", corner_idx)
}

fn is_piece_solved(state: &KState, orbit_name: &str, idx: usize) -> bool {
    let orbit = state
        .state_data
        .get(orbit_name)
        .expect("Invalid 3x3x3 state");
    // TODO: compare against the start state
    orbit.pieces[idx] == idx && orbit.orientation[idx] == 0
}

// TODO: allow comparing to state
pub fn is_f2l_solved(state: &KState) -> bool {
    let edges = state.state_data.get("EDGES").expect("Invalid 3x3x3 state");
    let corners = state
        .state_data
        .get("CORNERS")
        .expect("Invalid 3x3x3 state");
    let centers = state
        .state_data
        .get("CENTERS")
        .expect("Invalid 3x3x3 state");
    edges.pieces[4..12] == [4, 5, 6, 7, 8, 9, 10, 11]
        && edges.orientation[4..12] == [0, 0, 0, 0, 0, 0, 0, 0]
        && corners.pieces[4..8] == [4, 5, 6, 7]
        && corners.orientation[4..8] == [0, 0, 0, 0]
        && centers.pieces[0..2] == [0, 1] // We can get away with testing just two faces, and don't test orientation
}

pub fn is_3x3x3_cross_solved(state: &KState) -> bool {
    let edges = state.state_data.get("EDGES").expect("Invalid 3x3x3 state");
    edges.pieces[4..8] == [4, 5, 6, 7] && edges.orientation[4..8] == [0, 0, 0, 0]
}

// TODO: allow comparing to state
pub fn is_3x3x3_solved(state: &KState) -> bool {
    let edges = state.state_data.get("EDGES").expect("Invalid 3x3x3 state");
    let corners = state
        .state_data
        .get("CORNERS")
        .expect("Invalid 3x3x3 state");
    let centers = state
        .state_data
        .get("CENTERS")
        .expect("Invalid 3x3x3 state");
    edges.pieces == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        && edges.orientation == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        && corners.pieces[0..8] == [0, 1, 2, 3, 4, 5, 6, 7]
        && corners.orientation[0..8] == [0, 0, 0, 0, 0, 0, 0, 0]
        && centers.pieces[0..2] == [0, 1] // We can get away with testing just two faces, and don't test orientation
}
