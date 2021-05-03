use std::fmt;

struct Move {
  family: String,
  amount: i32
}

impl fmt::Display for Move {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.amount == 1 {
        write!(f, "{}", self.family)
    } else if self.amount == -1 {
        write!(f, "{}'", self.family)
    } else if self.amount < 0 {
        write!(f, "{}{}'", self.family, -self.amount)
    } else {
        write!(f, "{}{}", self.family, self.amount)
    }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("R", format!("{}", crate::Move {
            family: "R".to_string(),
            amount: 1
        }));
        assert_eq!("U2", format!("{}", crate::Move {
            family: "U".to_string(),
            amount: 2
        }));
        assert_eq!("R'", format!("{}", crate::Move {
            family: "R".to_string(),
            amount: -1
        }));
        assert_eq!("R0", format!("{}", crate::Move {
            family: "R".to_string(),
            amount: 0
        }));
        assert_eq!("R5", format!("{}", crate::Move {
            family: "R".to_string(),
            amount: 5
        }));
        assert_eq!("R12'", format!("{}", crate::Move {
            family: "R".to_string(),
            amount: -12
        }));
    }
}
