use std::{convert::TryFrom, fmt};

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

impl TryFrom<String> for Move {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.split_once(|c: char| c.is_digit(10)) {
            Some((family, amount_string)) => {
                let amount = amount_string
                    .parse()
                    .map_err(|err| format!("Invalid amount {amount_string}, error: {}", err))?;
                Ok(Move {
                    quantum: QuantumMove::new(family, None, None),
                    amount,
                })
            }
            None => Err("could not parse! 😱".into()),
        }

        // let amount_index = value.find(char::is_digit);
        // match amount_index {
        //     Some(index) => {
        //         let bla = 4;

        //     }
        //     None => Err("could not parse! 😱".into()),
        // }
        // if (match(None)) {

        // }
        // "mrkrhjewkhrwekrj4234"
    }
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
        // "UR43".to_string
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
