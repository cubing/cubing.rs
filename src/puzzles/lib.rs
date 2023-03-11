use crate::kpuzzle::KPuzzleDefinition;

fn cube3x3x3_definition() -> KPuzzleDefinition {
    let json_bytes = include_bytes!("3x3x3.kpuzzle.json");
    serde_json::from_slice(json_bytes).unwrap()
}

// TODO: Use IDs matching `cubing.js` even though the Rust convention is to capitalize?
pub enum PuzzleID {
    Cube3x3x3,
}

pub fn get_puzzle(puzzle_id: PuzzleID) -> KPuzzleDefinition {
    match puzzle_id {
        PuzzleID::Cube3x3x3 => cube3x3x3_definition(),
    }
}
