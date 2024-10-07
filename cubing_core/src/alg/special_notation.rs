use std::sync::Arc;

use super::QuantumMove;

#[allow(non_upper_case_globals)]
static U_SQ_quantum_cell: std::sync::OnceLock<Arc<QuantumMove>> = std::sync::OnceLock::new();
#[allow(non_snake_case)]
pub(crate) fn U_SQ_quantum() -> Arc<QuantumMove> {
    U_SQ_quantum_cell
        .get_or_init(|| {
            QuantumMove {
                family: "U_SQ_".to_owned(),
                prefix: None,
            }
            .into()
        })
        .clone()
}

#[allow(non_upper_case_globals)]
static D_SQ_quantum_cell: std::sync::OnceLock<Arc<QuantumMove>> = std::sync::OnceLock::new();
#[allow(non_snake_case)]
pub(crate) fn D_SQ_quantum() -> Arc<QuantumMove> {
    D_SQ_quantum_cell
        .get_or_init(|| {
            QuantumMove {
                family: "D_SQ_".to_owned(),
                prefix: None,
            }
            .into()
        })
        .clone()
}
