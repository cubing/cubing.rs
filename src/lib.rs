use std::{convert::TryFrom, fmt};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Move {
    pub quantum: QuantumMove,
    pub amount: isize,
}

impl Move {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, String> {
        match value.as_ref().split_once(|c: char| c.is_digit(10)) {
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
    }
}

impl TryFrom<String> for Move {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
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
    use std::convert::TryInto;

    use crate::Move;

    #[test]
    fn it_works() -> Result<(), String> {
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
        let mv: Move = "UR43".try_into()?;
        println!("Display: {}", mv);
        println!("Debug: {:?}", mv);
        Ok(())
    }
}
