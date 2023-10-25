use std::fmt;
use std::sync::Arc;

use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use super::amount::{fmt_amount, Amount};
use super::QuantumMove;

pub const _PLUSPLUS_: &str = "_PLUSPLUS_";

// TODO: Remove `PartialEq` if we add any metadata (e.g. parsing info, or memoizations).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub quantum: Arc<QuantumMove>,
    pub amount: Amount,
}

impl Move {
    pub fn invert(&self) -> Move {
        Self {
            quantum: Arc::clone(&self.quantum),
            amount: -self.amount,
        }
    }
}

// TODO: use https://docs.rs/serde_with/1.6.0/serde_with/index.html ?
impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Move {
    fn deserialize<D>(deserializer: D) -> Result<Move, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MoveVisitor)
    }
}

struct MoveVisitor;

impl<'de> Visitor<'de> for MoveVisitor {
    type Value = Move;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let r#move = s.parse::<Move>();
        match r#move {
            Ok(r#move) => Ok(r#move),
            Err(_) => Err(serde::de::Error::invalid_value(Unexpected::Str(s), &self)),
        }
    }
}

impl fmt::Display for Move {
    // TODO: memoize?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: memoize this calculation at construction time so we don't have to do it during every serialization?
        if let Some(family) = self.quantum.family.strip_suffix("_PLUSPLUS_") {
            let suffix = match self.amount {
                1 => "++",
                -1 => "--",
                _ => return Err(std::fmt::Error),
            };
            write!(
                f,
                "{}{}",
                QuantumMove {
                    family: family.to_owned(),
                    prefix: self.quantum.prefix.clone()
                },
                suffix
            )?;
            return Ok(());
        }
        write!(f, "{}", self.quantum)?;
        fmt_amount(f, self.amount)
    }
}
