use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::one_of,
    combinator::{all_consuming, into, map_res, opt},
    multi::many0,
    sequence::preceded,
    IResult,
};

use super::{
    alg_node::AlgNode, Alg, Commutator, Conjugate, Grouping, Move, MovePrefix, QuantumMove,
};

fn from_decimal_unsinged(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn from_natural_number_signed(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}

fn is_decimal_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_decimal_unsigned(input: &str) -> IResult<&str, u32> {
    map_res(take_while1(is_decimal_digit), from_decimal_unsinged)(input)
}

fn parse_natural_number_signed(input: &str) -> IResult<&str, i32> {
    map_res(take_while1(is_decimal_digit), from_natural_number_signed)(input)
}

// impl TryFrom<&str> for MoveLayer {
//     type Error = String;
//     fn try_from(input: &str) -> Result<Self, Self::Error> {
//         input.parse()
//     }
// }
// impl FromStr for MoveLayer {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match all_consuming(parse_decimal_unsigned)(s) {
//             Ok((_, move_layer)) => Ok(move_layer.into()),
//             Err(_) => Err("Invalid move layer string".into()),
//         }
//     }
// }

// fn parse_move_range(input: &str) -> IResult<&str, (u32, u32)> {
//     let (input, outer_layer) = parse_decimal_unsigned(input)?;
//     let (input, _) = tag("-")(input)?;
//     let (input, inner_layer) = parse_decimal_unsigned(input)?;
//     Ok((input, (outer_layer, inner_layer)))
// }
// impl TryFrom<&str> for MoveRange {
//     type Error = String;
//     fn try_from(input: &str) -> Result<Self, Self::Error> {
//         input.parse()
//     }
// }
// impl FromStr for MoveRange {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match all_consuming(parse_move_range)(s) {
//             Ok((_, layers)) => Ok(layers.into()),
//             Err(_) => Err("Invalid move range string".into()),
//         }
//     }
// }

fn parse_move_prefix(input: &str) -> IResult<&str, MovePrefix> {
    let (input, outer_layer) = parse_decimal_unsigned(input)?;
    let (input, dash) = opt(tag("-"))(input)?;
    if dash.is_none() {
        return Ok((input, outer_layer.into()));
    }
    let (input, inner_layer) = parse_decimal_unsigned(input)?;
    Ok((input, (outer_layer, inner_layer).into()))
}
// impl TryFrom<&str> for MovePrefix {
//     type Error = String;
//     fn try_from(input: &str) -> Result<Self, Self::Error> {
//         input.parse()
//     }
// }
// impl FromStr for MovePrefix {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match all_consuming(parse_move_prefix)(s) {
//             Ok((_, move_prefix)) => Ok(move_prefix),
//             Err(_) => Err("Invalid quantum move string".into()),
//         }
//     }
// }

fn is_family_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn parse_family(input: &str) -> IResult<&str, &str> {
    take_while1(is_family_char)(input)
}

fn parse_quantum_move(input: &str) -> IResult<&str, QuantumMove> {
    let (input, prefix) = opt(parse_move_prefix)(input)?;
    let (input, family) = parse_family(input)?;
    // let (input, amount) = parse_suffix(input)?;
    Ok((
        input,
        QuantumMove {
            family: family.to_owned(),
            prefix,
        },
    ))
}
impl TryFrom<&str> for QuantumMove {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for QuantumMove {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_quantum_move)(s) {
            Ok((_, q)) => Ok(q),
            Err(_) => Err("Invalid move range string".into()),
        }
    }
}

fn parse_amount_suffix(input: &str) -> IResult<&str, i32> {
    let (input, opt_amount) = opt(parse_natural_number_signed)(input)?;
    let (input, prime) = opt(tag("'"))(input)?;
    let mut amount = opt_amount.unwrap_or(1);
    if prime.is_some() {
        amount *= -1
    }
    Ok((input, amount))
}

fn parse_optional_amount_suffix(input: &str) -> IResult<&str, i32> {
    let (input, amount) = opt(parse_amount_suffix)(input)?;
    Ok((input, amount.unwrap_or(1)))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, quantum) = parse_quantum_move(input)?;
    let (input, amount) = parse_optional_amount_suffix(input)?;
    Ok((
        input,
        Move {
            quantum: quantum.into(),
            amount,
        },
    ))
}
impl TryFrom<&str> for Move {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_move)(s) {
            Ok((_, q)) => Ok(q),
            Err(_) => Err("Invalid move string".into()),
        }
    }
}

fn drop_spaces(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(tag(" "))(input)?;
    Ok((input, ()))
}

fn parse_alg(input: &str) -> IResult<&str, Alg> {
    let (input, nodes) = many0(preceded(drop_spaces, parse_node))(input)?;
    let (input, _) = drop_spaces(input)?;
    Ok((input, Alg { nodes }))
}
impl TryFrom<&str> for Alg {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for Alg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_alg)(s) {
            Ok((_, alg)) => Ok(alg),
            Err(_) => Err("Invalid move string".into()),
        }
    }
}

fn parse_grouping(input: &str) -> IResult<&str, Grouping> {
    let (input, _) = tag("(")(input)?;
    let (input, alg) = parse_alg(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, amount) = parse_optional_amount_suffix(input)?;
    Ok((
        input,
        Grouping {
            alg: alg.into(),
            amount,
        },
    ))
}

fn parse_commutator_or_conjugate(input: &str) -> IResult<&str, AlgNode> {
    let (input, _) = tag("[")(input)?;
    let (input, a) = parse_alg(input)?;
    let (input, separator) = one_of(",:")(input)?;
    let (input, b) = parse_alg(input)?;
    let (input, _) = tag("]")(input)?;
    let alg_node = if separator == ',' {
        Commutator {
            a: a.into(),
            b: b.into(),
        }
        .into()
    } else {
        Conjugate {
            a: a.into(),
            b: b.into(),
        }
        .into()
    };
    Ok((input, alg_node))
}

fn parse_node(input: &str) -> IResult<&str, AlgNode> {
    alt((
        into(parse_move),
        into(parse_grouping),
        into(parse_commutator_or_conjugate),
    ))(input)
}

// impl TryFrom<&str> for Grouping {
//     type Error = String;
//     fn try_from(input: &str) -> Result<Self, Self::Error> {
//         input.parse()
//     }
// }
// impl FromStr for Grouping {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match all_consuming(parse_grouping)(s) {
//             Ok((_, grouping)) => Ok(grouping),
//             Err(_) => Err("Invalid move string".into()),
//         }
//     }
// }
