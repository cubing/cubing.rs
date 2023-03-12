use cubing::{
    alg::Alg,
    kpuzzle::{KPuzzle, KTransformation},
    parse_alg,
};

pub enum F2LSlot {
    H,
    I,
    J,
    K,
}

// We'd place this `allow` directly about `HIJK_alg`, but that doesn't work. See https://github.com/divviup/libprio-rs/pull/445/files
#[allow(non_snake_case)]
pub struct TriggerInfo {
    pub short_alg: Alg,
    pub long_alg: Alg,
    pub transformation: KTransformation,
}

impl TriggerInfo {
    #[allow(non_snake_case)]
    pub fn new(kpuzzle: &KPuzzle, short_alg: &str, long_alg: &str) -> TriggerInfo {
        let short_alg = parse_alg!(short_alg).unwrap();
        let long_alg = parse_alg!(long_alg).unwrap();
        let transformation = kpuzzle.transformation_from_alg(&long_alg).unwrap();
        TriggerInfo {
            short_alg,
            long_alg,
            transformation,
        }
    }
}

pub struct SlotTriggerInfo {
    pub f2l_slot: F2LSlot,
    pub triggers: Vec<TriggerInfo>,
}

pub fn get_triggers_by_slot(kpuzzle: &KPuzzle) -> Vec<SlotTriggerInfo> {
    vec![
        SlotTriggerInfo {
            f2l_slot: F2LSlot::K,
            triggers: vec![
                TriggerInfo::new(kpuzzle, "K", "R U R'"),
                TriggerInfo::new(kpuzzle, "K2", "R U2 R'"),
                TriggerInfo::new(kpuzzle, "K'", "R U' R'"),
                TriggerInfo::new(kpuzzle, "FK", "F' U' F"),
                TriggerInfo::new(kpuzzle, "FK2", "F' U2 F"),
                TriggerInfo::new(kpuzzle, "FK'", "F' U F"),
            ],
        },
        SlotTriggerInfo {
            f2l_slot: F2LSlot::J,
            triggers: vec![
                TriggerInfo::new(kpuzzle, "J", "R' U' R"),
                TriggerInfo::new(kpuzzle, "J2", "R' U2 R"),
                TriggerInfo::new(kpuzzle, "J'", "R' U R"),
                TriggerInfo::new(kpuzzle, "BJ", "B U B'"),
                TriggerInfo::new(kpuzzle, "BJ2", "B U2 B'"),
                TriggerInfo::new(kpuzzle, "BJ'", "B U' B'"),
            ],
        },
        SlotTriggerInfo {
            f2l_slot: F2LSlot::H,
            triggers: vec![
                TriggerInfo::new(kpuzzle, "H", "L' U' L"),
                TriggerInfo::new(kpuzzle, "H2", "L' U2 L"),
                TriggerInfo::new(kpuzzle, "H'", "L' U L"),
                TriggerInfo::new(kpuzzle, "FH", "F U F'"),
                TriggerInfo::new(kpuzzle, "FH2", "F U2 F'"),
                TriggerInfo::new(kpuzzle, "FH'", "F U' F'"),
            ],
        },
        SlotTriggerInfo {
            f2l_slot: F2LSlot::I,
            triggers: vec![
                TriggerInfo::new(kpuzzle, "I", "L U L'"),
                TriggerInfo::new(kpuzzle, "I2", "L U2 L'"),
                TriggerInfo::new(kpuzzle, "I'", "L U' L'"),
                TriggerInfo::new(kpuzzle, "BI", "B' U' B"),
                TriggerInfo::new(kpuzzle, "BI2", "B' U2 B"),
                TriggerInfo::new(kpuzzle, "BI'", "B' U B"),
            ],
        },
    ]
}

pub fn get_auf_triggers(kpuzzle: &KPuzzle) -> Vec<TriggerInfo> {
    vec![
        TriggerInfo::new(kpuzzle, "", ""),
        TriggerInfo::new(kpuzzle, "U", "U"),
        TriggerInfo::new(kpuzzle, "U2", "U2"),
        TriggerInfo::new(kpuzzle, "U'", "U'"),
    ]
}
