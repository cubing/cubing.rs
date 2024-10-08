use std::{fmt::Display, str::FromStr, sync::Arc};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::one_of,
    combinator::{all_consuming, into, map_res, opt},
    multi::{many0, many1},
    sequence::pair,
    IResult,
};

use super::{
    alg_node::AlgNode,
    r#move::{_PLUSPLUS_, _PLUS_},
    special_notation::{D_SQ_quantum, U_SQ_quantum},
    Alg, Commutator, Conjugate, Grouping, LineComment, Move, MovePrefix, Newline, Pause,
    QuantumMove,
};

#[derive(Debug)]
/// Could not parse an alg (or alg part), usually due to invalid syntax.
pub struct AlgParseError {
    pub description: String,
}

// TODO: is Rust smart enough to optimize this using just the `From<&str>` definition?
impl From<String> for AlgParseError {
    fn from(description: String) -> Self {
        Self { description }
    }
}

impl From<&str> for AlgParseError {
    fn from(description: &str) -> Self {
        Self {
            description: description.to_owned(),
        }
    }
}

impl Display for AlgParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

fn from_decimal_unsigned(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn from_natural_number_signed(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}

fn is_decimal_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_decimal_unsigned(input: &str) -> IResult<&str, u32> {
    map_res(take_while1(is_decimal_digit), from_decimal_unsigned)(input)
}

fn parse_natural_number_signed(input: &str) -> IResult<&str, i32> {
    map_res(take_while1(is_decimal_digit), from_natural_number_signed)(input)
}

fn parse_negative_decimal_signed(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag("-")(input)?;
    let (input, n) = parse_natural_number_signed(input)?;
    // TODO: check cast?
    Ok((input, -n))
}

fn decimal_signed(input: &str) -> IResult<&str, i32> {
    alt((parse_natural_number_signed, parse_negative_decimal_signed))(input)
}

fn parse_move_prefix(input: &str) -> IResult<&str, MovePrefix> {
    let (input, outer_layer) = parse_decimal_unsigned(input)?;
    let (input, dash) = opt(tag("-"))(input)?;
    if dash.is_none() {
        return Ok((input, outer_layer.into()));
    }
    let (input, inner_layer) = parse_decimal_unsigned(input)?;
    Ok((input, (outer_layer, inner_layer).into()))
}

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
    type Error = AlgParseError;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for QuantumMove {
    type Err = AlgParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_quantum_move)(s) {
            Ok((_, q)) => Ok(q),
            Err(_) => Err("Invalid move range string".into()),
        }
    }
}

fn parse_optional_positive_amount_number(input: &str) -> IResult<&str, Option<i32>> {
    let (input, amount_number) = opt(parse_natural_number_signed)(input)?;
    Ok((input, amount_number))
}

fn parse_optional_prime(input: &str) -> IResult<&str, i32> {
    let (input, prime) = opt(tag("'"))(input)?;
    Ok((
        input,
        match prime {
            Some(_) => -1,
            None => 1,
        },
    ))
}

fn parse_optional_amount_suffix(input: &str) -> IResult<&str, i32> {
    let (input, amount_number) = parse_optional_positive_amount_number(input)?;
    let amount_number = amount_number.unwrap_or(1);
    let (input, prime) = parse_optional_prime(input)?;
    Ok((input, amount_number * prime))
}

enum PochmannStringSuffix {
    PlusPlus,
    MinusMinus,
}

impl PochmannStringSuffix {
    fn amount(&self) -> i32 {
        match self {
            PochmannStringSuffix::PlusPlus => 1,
            PochmannStringSuffix::MinusMinus => -1,
        }
    }

    fn add_internal_suffix(quantum: QuantumMove) -> QuantumMove {
        QuantumMove {
            family: format!("{}{}", quantum.family, _PLUSPLUS_),
            prefix: quantum.prefix,
        }
    }
}

enum ClockSuffix {
    Plus,
    Minus,
}

impl ClockSuffix {
    fn amount(&self) -> i32 {
        match self {
            ClockSuffix::Plus => 1,
            ClockSuffix::Minus => -1,
        }
    }

    fn add_internal_suffix(quantum: QuantumMove) -> QuantumMove {
        QuantumMove {
            family: format!("{}{}", quantum.family, _PLUS_),
            prefix: quantum.prefix,
        }
    }
}

fn parse_pochmann_megaminx_suffix(input: &str) -> IResult<&str, PochmannStringSuffix> {
    let (input, suffix) = alt((tag("++"), tag("--")))(input)?;
    Ok((
        input,
        match suffix {
            "++" => PochmannStringSuffix::PlusPlus,
            _ => PochmannStringSuffix::MinusMinus,
        },
    ))
}

fn parse_clock_suffix(input: &str) -> IResult<&str, ClockSuffix> {
    let (input, suffix) = alt((tag("+"), tag("-")))(input)?;
    Ok((
        input,
        match suffix {
            "+" => ClockSuffix::Plus,
            _ => ClockSuffix::Minus,
        },
    ))
}

fn parse_slash(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("/")(input)?;
    Ok((input, ()))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, slash) = opt(parse_slash)(input)?;
    if slash.is_some() {
        return Ok((
            input,
            Move {
                quantum: Arc::new(QuantumMove {
                    family: "_SLASH_".to_owned(),
                    prefix: None,
                }),
                amount: 1,
            },
        ));
    }

    let (input, quantum) = parse_quantum_move(input)?;
    let (input, pochmann_suffix) = opt(parse_pochmann_megaminx_suffix)(input)?;
    if let Some(pochmann_suffix) = pochmann_suffix {
        let amount = pochmann_suffix.amount();
        return Ok((
            input,
            Move {
                quantum: Arc::new(PochmannStringSuffix::add_internal_suffix(quantum)),
                amount,
            },
        ));
    }

    let (input, amount_number) = parse_optional_positive_amount_number(input)?;

    if let Some(amount_number) = amount_number {
        let (input, clock_suffix) = opt(parse_clock_suffix)(input)?;
        if let Some(clock_suffix) = clock_suffix {
            let amount = amount_number * clock_suffix.amount();
            return Ok((
                input,
                Move {
                    quantum: Arc::new(ClockSuffix::add_internal_suffix(quantum)),
                    amount,
                },
            ));
        }
    };

    let (input, prime) = parse_optional_prime(input)?;

    Ok((
        input,
        Move {
            quantum: quantum.into(),
            amount: amount_number.unwrap_or(1) * prime,
        },
    ))
}
impl TryFrom<&str> for Move {
    type Error = AlgParseError;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for Move {
    type Err = AlgParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_move)(s) {
            Ok((_, q)) => Ok(q),
            Err(_) => Err("Invalid move string".into()),
        }
    }
}

fn parse_potential_spaces(input: &str) -> IResult<&str, bool> {
    let (input, spaces) = opt(many1(tag(" ")))(input)?;
    Ok((input, spaces.is_some()))
}

fn parse_alg(input: &str) -> IResult<&str, Alg> {
    let (input, spaces_and_nodes) = many0(pair(parse_potential_spaces, parse_node))(input)?;
    let (input, _) = parse_potential_spaces(input)?;

    let mut previous_node: Option<&AlgNode> = None;
    for spaces_and_node in &spaces_and_nodes {
        let (preceded_by_spaces, node) = spaces_and_node;

        if !matches!(node, AlgNode::NewlineNode(_)) {
            let is_crowded = !*preceded_by_spaces
                && match previous_node {
                    Some(AlgNode::NewlineNode(_)) => false,
                    Some(AlgNode::LineCommentNode(_)) => false,
                    None => false,
                    Some(_) => true,
                };

            if is_crowded {
                // TODO issue a useful error message
                one_of(" \n")(input)?; // TODO: is there a way to do this without re-parsing?
            }
        }

        previous_node = Some(node);
    }

    let nodes: Vec<AlgNode> = spaces_and_nodes.into_iter().map(|(_, node)| node).collect();
    Ok((input, Alg { nodes }))
}

impl TryFrom<&str> for Alg {
    type Error = AlgParseError;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        input.parse()
    }
}
impl FromStr for Alg {
    type Err = AlgParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_alg)(s) {
            Ok((_, alg)) => Ok(alg),
            Err(s) => {
                eprintln!("Alg parsing error: {}", s);
                Err("Invalid alg string".into()) // TODO: pass on error
            }
        }
    }
}

fn parse_pause(input: &str) -> IResult<&str, Pause> {
    let (input, _) = tag(".")(input)?;
    Ok((input, Pause {}))
}

// TODO: support `\r`?
fn parse_newline(input: &str) -> IResult<&str, Newline> {
    let (input, _) = tag("\n")(input)?;
    Ok((input, Newline {}))
}

// fn parse_newline_or_eof(input: &str) -> IResult<&str, ()> {
//     if let Ok((input, _)) = parse_newline(input) {
//         return Ok((input, ()));
//     };
//     let (input, _) = eof(input)?;
//     Ok((input, ()))
// }

fn parse_line_comment(input: &str) -> IResult<&str, LineComment> {
    let (input, _) = tag("//")(input)?;
    let (input, text) = take_till(|c| c == '\n')(input)?;
    let line_comment = LineComment::try_new(text).unwrap(); // TODO: is there an idiomatic way to avoid the need to unwrap?
    Ok((input, line_comment))
}

fn parse_grouping_rest(input: &str) -> IResult<&str, Grouping> {
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

fn parse_square1_tuple_rest(input: &str) -> IResult<&str, Grouping> {
    let (input, top_amount) = decimal_signed(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, bottom_amount) = decimal_signed(input)?;
    let (input, _) = tag(")")(input)?;
    let alg = Alg {
        nodes: vec![
            AlgNode::MoveNode(Move {
                quantum: U_SQ_quantum(),
                amount: top_amount,
            }),
            AlgNode::MoveNode(Move {
                quantum: D_SQ_quantum(),
                amount: bottom_amount,
            }),
        ],
    };
    Ok((
        input,
        Grouping {
            alg: alg.into(),
            amount: 1,
        },
    ))
}

fn parse_grouping(input: &str) -> IResult<&str, Grouping> {
    let (input, _) = tag("(")(input)?;
    alt((into(parse_grouping_rest), into(parse_square1_tuple_rest)))(input)
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
    let (input, amount) = parse_optional_amount_suffix(input)?;
    if amount == 1 {
        return Ok((input, alg_node));
    }
    let grouping = Grouping {
        alg: Alg {
            nodes: vec![alg_node],
        }
        .into(),
        amount,
    };
    Ok((input, grouping.into()))
}

fn parse_node(input: &str) -> IResult<&str, AlgNode> {
    alt((
        into(parse_line_comment), // Placed before `parse_move` to parse `//` before `/`.
        into(parse_move),
        into(parse_pause),
        into(parse_newline),
        into(parse_grouping),
        into(parse_commutator_or_conjugate),
    ))(input)
}
