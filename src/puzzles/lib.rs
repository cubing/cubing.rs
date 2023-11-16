use crate::kpuzzle::{KPuzzleDefinition, PackedKPuzzle};

// TODO: avoid re-parsing every time
pub fn cube3x3x3_kpuzzle() -> PackedKPuzzle {
    let json_bytes = include_bytes!("3x3x3.kpuzzle.json");
    let def: KPuzzleDefinition = serde_json::from_slice(json_bytes).unwrap();
    PackedKPuzzle::try_new(def).unwrap()
}

// TODO: avoid re-parsing every time
pub fn cube2x2x2_kpuzzle() -> PackedKPuzzle {
    let json_bytes = include_bytes!("2x2x2.kpuzzle.json");
    let def: KPuzzleDefinition = serde_json::from_slice(json_bytes).unwrap();
    PackedKPuzzle::try_new(def).unwrap()
}
