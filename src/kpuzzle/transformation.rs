use std::collections::HashMap;

pub struct KTransformation {
    pub transformation_data: KTransformationData, // TODO: check that this is immutable
}
pub type KTransformationData = HashMap<String, KTransformationOrbitData>;
pub struct KTransformationOrbitData {
    pub permutation: Vec<u32>,
    pub orientation: Vec<u32>,
}
