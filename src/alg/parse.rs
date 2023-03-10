use std::str::FromStr;

use nom::{
    bytes::complete::take_while,
    combinator::{all_consuming, map_res},
    IResult,
};

use super::MoveLayer;

fn from_decimal(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn is_decimal_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_decimal(input: &str) -> IResult<&str, u32> {
    map_res(take_while(is_decimal_digit), from_decimal)(input)
}

impl TryFrom<&str> for MoveLayer {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for MoveLayer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_decimal)(s) {
            Ok((_, move_layer)) => Ok(move_layer.into()),
            Err(_) => Err("Invalid move layer".into()),
        }
    }
}
