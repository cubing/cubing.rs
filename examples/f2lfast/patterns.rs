use cubing::kpuzzle::KPattern;

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
    pub fn is_f2l_solved(&self) -> bool {
        self.H && self.I && self.J && self.K
    }
    pub fn is_slot_solved(&self, f2l_slot: &F2LSlot) -> bool {
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

    // Does *not* check if cross is solved.
    pub fn from_pattern(pattern: &KPattern) -> Self {
        Self {
            H: is_slot_solved(pattern, &F2LSlot::H),
            I: is_slot_solved(pattern, &F2LSlot::I),
            J: is_slot_solved(pattern, &F2LSlot::J),
            K: is_slot_solved(pattern, &F2LSlot::K),
        }
    }
}

// TODO: is it more efficient not to borrow `F2LSlot`?
pub fn is_slot_solved(pattern: &KPattern, f2l_slot: &F2LSlot) -> bool {
    // Reid order:
    // UF  UR  UB  UL  . DF  DR  DB  DL  . FR  FL  BR  BL
    // UFR URB UBL ULF . DRF DFL DLB DBR
    // U L F R B D
    match f2l_slot {
        F2LSlot::H => are_slot_pieces_solved(pattern, 9, 5),
        F2LSlot::I => are_slot_pieces_solved(pattern, 11, 6),
        F2LSlot::J => are_slot_pieces_solved(pattern, 10, 7),
        F2LSlot::K => are_slot_pieces_solved(pattern, 8, 4),
    }
}

const ORBIT_INDEX_EDGES: usize = 0;
const ORBIT_INDEX_CORNERS: usize = 0;

pub fn are_slot_pieces_solved(pattern: &KPattern, edge_idx: u8, corner_idx: u8) -> bool {
    is_piece_solved(pattern, ORBIT_INDEX_EDGES, edge_idx)
        && is_piece_solved(pattern, ORBIT_INDEX_CORNERS, corner_idx)
}

fn is_piece_solved(pattern: &KPattern, orbit_index: usize, idx: u8) -> bool {
    let orbit_info = &pattern.kpuzzle().data.orbit_iteration_info[orbit_index];
    // TODO: compare against the start pattern
    pattern.get_piece(orbit_info, idx) == idx
        && pattern
            .get_orientation_with_mod(orbit_info, idx)
            .orientation
            == 0
}

// // TODO: allow comparing to pattern
// pub fn is_f2l_solved(pattern: &KPattern) -> bool {
//     let edges = pattern.pattern_data.get("EDGES").expect("Invalid 3x3x3 pattern");
//     let corners = pattern
//         .pattern_data
//         .get("CORNERS")
//         .expect("Invalid 3x3x3 pattern");
//     let centers = pattern
//         .pattern_data
//         .get("CENTERS")
//         .expect("Invalid 3x3x3 pattern");
//     edges.pieces[4..12] == [4, 5, 6, 7, 8, 9, 10, 11]
//         && edges.orientation[4..12] == [0, 0, 0, 0, 0, 0, 0, 0]
//         && corners.pieces[4..8] == [4, 5, 6, 7]
//         && corners.orientation[4..8] == [0, 0, 0, 0]
//         && centers.pieces[0..2] == [0, 1] // We can get away with testing just two faces, and don't test orientation
// }

pub fn is_3x3x3_cross_solved(pattern: &KPattern) -> bool {
    is_piece_solved(pattern, ORBIT_INDEX_EDGES, 4)
        && is_piece_solved(pattern, ORBIT_INDEX_EDGES, 5)
        && is_piece_solved(pattern, ORBIT_INDEX_EDGES, 6)
        && is_piece_solved(pattern, ORBIT_INDEX_EDGES, 7)
}

// TODO: allow comparing to pattern
pub fn is_3x3x3_solved(pattern: &KPattern) -> bool {
    pattern == &pattern.kpuzzle().default_pattern()
    // let edges = pattern
    //     .kpattern_data
    //     .get(&("EDGES").into())
    //     .expect("Invalid 3x3x3 pattern");
    // let corners = pattern
    //     .kpattern_data
    //     .get(&("CORNERS").into())
    //     .expect("Invalid 3x3x3 pattern");
    // let centers = pattern
    //     .kpattern_data
    //     .get(&("CENTERS").into())
    //     .expect("Invalid 3x3x3 pattern");
    // edges.pieces == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    //     && edges.orientation == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    //     && corners.pieces[0..8] == [0, 1, 2, 3, 4, 5, 6, 7]
    //     && corners.orientation[0..8] == [0, 0, 0, 0, 0, 0, 0, 0]
    //     && centers.pieces[0..2] == [0, 1] // We can get away with testing just two faces, and don't test orientation
}
