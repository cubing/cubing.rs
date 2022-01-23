use std::fmt;

pub struct QuantumMove {
    pub family: String,
    pub outer_layer: Option<usize>,
    pub inner_layer: Option<usize>,
}

impl fmt::Display for QuantumMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.family)
    }
}

impl QuantumMove {
    pub fn new(
        family: impl Into<String>,
        outer_layer: Option<usize>,
        inner_layer: Option<usize>,
    ) -> Self {
        Self {
            family: family.into(),
            outer_layer,
            inner_layer,
        }
    }
}

pub struct Move {
    pub quantum: QuantumMove,
    pub amount: isize,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.amount == 1 {
            write!(f, "{}", self.quantum)
        } else if self.amount == -1 {
            write!(f, "{}'", self.quantum)
        } else if self.amount < 0 {
            write!(f, "{}{}'", self.quantum, -self.amount)
        } else {
            write!(f, "{}{}", self.quantum, self.amount)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            "R",
            format!(
                "{}",
                crate::Move {
                    quantum: crate::QuantumMove::new("R", None, None),
                    amount: 1
                }
            )
        );
        // assert_eq!(
        //     "U2",
        //     format!(
        //         "{}",
        //         crate::Move {
        //             family: "U".to_string(),
        //             amount: 2
        //         }
        //     )
        // );
        // assert_eq!(
        //     "R'",
        //     format!(
        //         "{}",
        //         crate::Move {
        //             family: "R".to_string(),
        //             amount: -1
        //         }
        //     )
        // );
        // assert_eq!(
        //     "R0",
        //     format!(
        //         "{}",
        //         crate::Move {
        //             family: "R".to_string(),
        //             amount: 0
        //         }
        //     )
        // );
        // assert_eq!(
        //     "R5",
        //     format!(
        //         "{}",
        //         crate::Move {
        //             family: "R".to_string(),
        //             amount: 5
        //         }
        //     )
        // );
        // assert_eq!(
        //     "R12'",
        //     format!(
        //         "{}",
        //         crate::Move {
        //             family: "R".to_string(),
        //             amount: -12
        //         }
        //     )
        // );
    }
}
