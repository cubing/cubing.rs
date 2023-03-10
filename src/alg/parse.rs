use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_while},
    combinator::{all_consuming, map_res},
    IResult,
};

use super::{MoveLayer, MovePrefix, MoveRange};

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

pub fn parse_move_range(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, outer_layer) = parse_decimal(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, inner_layer) = parse_decimal(input)?;
    Ok((input, (outer_layer, inner_layer)))
}
impl TryFrom<&str> for MoveRange {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for MoveRange {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_move_range)(s) {
            Ok((_, layers)) => Ok(layers.into()),
            Err(_) => Err("Invalid move range".into()),
        }
    }
}

pub fn parse_move_prefix(input: &str) -> IResult<&str, MovePrefix> {
    let (input, outer_layer) = parse_decimal(input)?;
    let (input, _) = match tag::<&str, &str, ()>("-")(input) {
        Ok(a) => a,
        Err(_) => {
            return Ok((input, outer_layer.into()));
        }
    };
    let (input, inner_layer) = parse_decimal(input)?;
    Ok((input, (outer_layer, inner_layer).into()))
}
impl TryFrom<&str> for MovePrefix {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for MovePrefix {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_move_prefix)(s) {
            Ok((_, move_prefix)) => Ok(move_prefix),
            Err(_) => Err("Invalid move prefix".into()),
        }
    }
}
