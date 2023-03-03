use std::collections::HashMap;

pub type KStateData = HashMap<String, KStateOrbitData>;
pub struct KStateOrbitData {
    pub pieces: Vec<u32>,
    pub orientation: Vec<u32>,
}

pub struct KState {
    pub state_data: KStateData,
}
