use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type KStateData = HashMap<String, KStateOrbitData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct KStateOrbitData {
    pub pieces: Vec<u32>,
    pub orientation: Vec<u32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KState {
    pub state_data: KStateData,
}
