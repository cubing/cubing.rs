use crate::kpuzzle::{KPuzzle, KPuzzleDefinition};

// TODO: avoid re-parsing every time
pub fn cube3x3x3_kpuzzle() -> KPuzzle {
    let json_bytes = include_bytes!("3x3x3.kpuzzle.json");
    let def: KPuzzleDefinition = serde_json::from_slice(json_bytes).unwrap();
    def.into()
}
