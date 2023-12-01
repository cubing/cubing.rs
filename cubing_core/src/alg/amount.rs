use std::fmt;

pub type Amount = i32;

// TODO: figure out how to turn this into a struct/enum/trait, without making it annoying to access the amount of a node.
pub fn fmt_amount(f: &mut fmt::Formatter<'_>, amount: Amount) -> fmt::Result {
    if amount == 1 {
        write!(f, "")
    } else if amount == -1 {
        write!(f, "'")
    } else if amount < 0 {
        write!(f, "{}'", -amount)
    } else {
        write!(f, "{}", amount)
    }
}
