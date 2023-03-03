use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;
use std::{convert::TryFrom, fmt};

use crate::alg::QuantumMove;

// TODO: figure out whether to hash the string
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub quantum: Rc<QuantumMove>,
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
        if captures.name("prime").is_some() {
            amount *= -1
        };

        Ok(Move {
            quantum: QuantumMove::new(family, outer_layer, inner_layer).into(),
            amount,
        })
    }

    pub fn invert(&self) -> Move {
        Self {
            quantum: Rc::clone(&self.quantum),
            amount: -self.amount,
        }
    }
    // from_str?
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
    // TODO: memoize?
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
