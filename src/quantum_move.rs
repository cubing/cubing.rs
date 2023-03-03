use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumMove {
    pub family: String,
    // TODO: prevent setting outer layer without inner layer
    pub outer_layer: Option<u32>,
    pub inner_layer: Option<u32>,
}

impl fmt::Display for QuantumMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let layer_string = match self.inner_layer {
            Some(inner_layer) => {
                let inner_layer_str = inner_layer.to_string();
                match self.outer_layer {
                    Some(outer_layer) => outer_layer.to_string() + "-" + &inner_layer_str,
                    None => inner_layer_str,
                }
            }
            None => "".into(),
        };
        write!(f, "{}{}", layer_string, self.family)
    }
}

impl QuantumMove {
    pub fn new(
        family: impl Into<String>,
        outer_layer: Option<u32>,
        inner_layer: Option<u32>,
    ) -> Self {
        Self {
            family: family.into(),
            outer_layer,
            inner_layer,
        }
    }
}
