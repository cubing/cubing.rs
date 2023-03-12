use cubing::{
    alg::Alg,
    kpuzzle::{KPuzzle, KTransformation},
    parse_alg,
};

// We'd place this `allow` directly about `HIJK_alg`, but that doesn't work. See https://github.com/divviup/libprio-rs/pull/445/files
#[allow(non_snake_case)]
pub struct TriggerInfo {
    pub HIJK_alg: Alg,
    pub expanded_alg: Alg,
    pub transformation: KTransformation,
}

impl TriggerInfo {
    #[allow(non_snake_case)]
    pub fn new(kpuzzle: &KPuzzle, HIJK_alg: &str, expanded_alg: &str) -> TriggerInfo {
        let HIJK_alg = parse_alg!(HIJK_alg).unwrap();
        let expanded_alg = parse_alg!(expanded_alg).unwrap();
        let transformation = kpuzzle.transformation_from_alg(&expanded_alg).unwrap();
        TriggerInfo {
            HIJK_alg,
            expanded_alg,
            transformation,
        }
    }
}

pub fn get_triggers(kpuzzle: &KPuzzle) -> Vec<TriggerInfo> {
    vec![
        TriggerInfo::new(kpuzzle, "H", "L' U' L"),
        TriggerInfo::new(kpuzzle, "H2", "L' U2 L"),
        TriggerInfo::new(kpuzzle, "H'", "L' U L"),
        TriggerInfo::new(kpuzzle, "I", "L U L'"),
        TriggerInfo::new(kpuzzle, "I2", "L U2 L'"),
        TriggerInfo::new(kpuzzle, "I'", "L U' L'"),
        TriggerInfo::new(kpuzzle, "J", "R' U' R"),
        TriggerInfo::new(kpuzzle, "J2", "R' U2 R"),
        TriggerInfo::new(kpuzzle, "J'", "R' U R"),
        TriggerInfo::new(kpuzzle, "K", "R U' R'"),
        TriggerInfo::new(kpuzzle, "K2", "R U2 R'"),
        TriggerInfo::new(kpuzzle, "K'", "R U R'"),
    ]
}
