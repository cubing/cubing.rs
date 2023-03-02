use lazy_static::lazy_static;
use regex::Regex;
use std::{convert::TryFrom, fmt};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Move {
    pub quantum: QuantumMove,
    pub amount: i32,
}

impl Move {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?:(?:(?P<outer_layer>[1-9]\d*)-)?(?P<inner_layer>[1-9]\d*))?(?P<family>[a-zA-Z_]+)(?P<amount>\d+)?(?P<prime>')?$").unwrap();
        }
        let captures = match RE.captures(value.as_ref()) {
            Some(captures) => captures,
            None => return Err("could not parse! 😱".into()),
        };

        let outer_layer = match captures.name("outer_layer") {
            Some(outer_layer_match) => match outer_layer_match.as_str().parse::<u32>() {
                Ok(outer_layer) => Some(outer_layer),
                Err(_) => return Err("Could not parse outer layer".into()),
            },
            None => None,
        };

        let inner_layer = match captures.name("inner_layer") {
            Some(inner_layer_match) => match inner_layer_match.as_str().parse::<u32>() {
                Ok(inner_layer) => Some(inner_layer),
                Err(_) => return Err("Could not parse inner layer".into()),
            },
            None => None,
        };

        let family = captures.name("family").unwrap().as_str();

        let mut amount = match captures.name("amount") {
            Some(amount_match) => match amount_match.as_str().parse::<i32>() {
                Ok(amount) => amount,
                Err(_) => return Err("Could not parse move amount".into()),
            },
            None => 1,
        };
        match captures.name("prime") {
            Some(_) => amount *= -1,
            None => {}
        };

        Ok(Move {
            quantum: QuantumMove::new(family, outer_layer, inner_layer),
            amount: amount,
        })
    }

    pub fn invert(&self) -> Move {
        Self {
            quantum: self.quantum.clone(),
            amount: -self.amount,
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
        assert_eq!("F2", format!("{}", crate::Move::parse("F2").unwrap()));
        assert_eq!("F", format!("{}", crate::Move::parse("F1").unwrap()));
        assert_eq!("F", format!("{}", crate::Move::parse("F").unwrap()));
        assert_eq!("F'", format!("{}", crate::Move::parse("F1'").unwrap()));
        assert_eq!("F0", format!("{}", crate::Move::parse("F0").unwrap()));
        assert_eq!("F2'", format!("{}", crate::Move::parse("F2'").unwrap()));
        assert_eq!("U_R", format!("{}", crate::Move::parse("U_R").unwrap()));
        assert_eq!("4R2'", format!("{}", crate::Move::parse("4R2'").unwrap()));
        assert_eq!(
            "3-7R2'",
            format!("{}", crate::Move::parse("3-7R2'").unwrap())
        );

        assert_eq!(
            "3-7R2'",
            format!(
                "{}",
                crate::Move {
                    quantum: crate::QuantumMove::new("R", Some(3), Some(7)),
                    amount: -2
                }
            )
        );

        let single_move = crate::Move::parse("R2'").unwrap();
        assert_eq!(single_move.quantum.outer_layer, None);
        assert_eq!(single_move.quantum.inner_layer, None);
        assert_eq!(single_move.quantum.family, "R");
        assert_eq!(single_move.amount, -2);

        let face_move = crate::Move::parse("R2'").unwrap();
        assert_eq!(face_move.quantum.outer_layer, None);
        assert_eq!(face_move.quantum.inner_layer, None);
        assert_eq!(face_move.quantum.family, "R");
        assert_eq!(face_move.amount, -2);

        let block_move = crate::Move::parse("7R2'").unwrap();
        assert_eq!(block_move.quantum.outer_layer, None);
        assert_eq!(block_move.quantum.inner_layer, Some(7));
        assert_eq!(block_move.quantum.family, "R");
        assert_eq!(block_move.amount, -2);

        let range_move = crate::Move::parse("3-7R2'").unwrap();
        assert_eq!(range_move.quantum.outer_layer, Some(3));
        assert_eq!(range_move.quantum.inner_layer, Some(7));
        assert_eq!(range_move.quantum.family, "R");
        assert_eq!(range_move.amount, -2);

        assert_eq!(
            "F2'",
            format!("{}", crate::Move::parse("F2").unwrap().invert())
        );

        assert!(crate::Move::parse("2").is_err());
        assert!(crate::Move::parse("U-R").is_err());
        let mv: Move = "UR43".try_into()?;
        println!("Display: {}", mv);
        println!("Debug: {:?}", mv);
        Ok(())
    }
}
