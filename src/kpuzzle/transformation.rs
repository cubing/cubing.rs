use std::{collections::HashMap, rc::Rc};

pub struct KTransformation {
    // TODO: store the orbits directly?
    pub transformation_data: Rc<KTransformationData>, // TODO: check that this is immutable
}
pub type KTransformationData = HashMap<String, KTransformationOrbitData>;
pub struct KTransformationOrbitData {
    pub permutation: Vec<u32>,
    pub orientation: Vec<u32>,
}
