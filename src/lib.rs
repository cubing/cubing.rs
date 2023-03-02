use lazy_static::lazy_static;
use regex::Regex;
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
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([a-zA-Z_]+)(\d+)?$").unwrap();
        }
        let captures = match RE.captures(value.as_ref()) {
            Some(captures) => captures,
            None => return Err("could not parse! 😱".into()),
        };
        let amount = match captures.get(2) {
            Some(amount_match) => match amount_match.as_str().parse::<isize>() {
                Ok(amount) => amount,
                Err(_) => return Err("Could not parse move amount".into()),
            },
            None => 1,
        };
        Ok(Move {
            quantum: QuantumMove::new(&captures[1], None, None),
            amount: amount,
        })
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
        assert_eq!("F2", format!("{}", crate::Move::parse("F2").unwrap()));
        assert_eq!("F", format!("{}", crate::Move::parse("F1").unwrap()));
        assert_eq!("F", format!("{}", crate::Move::parse("F").unwrap()));
        assert_eq!("F", format!("{}", crate::Move::parse("F1").unwrap()));
        assert!(crate::Move::parse("2").is_err());
        let mv: Move = "UR43".try_into()?;
        println!("Display: {}", mv);
        println!("Debug: {:?}", mv);
        Ok(())
    }
}
