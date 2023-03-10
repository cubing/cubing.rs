use super::Move;

pub struct Alg {
    pub nodes: Vec<Move>,
}
// use nom::{bytes::complete::take_while, character::is_space, sequence::preceded};

impl Alg {
    pub fn invert(&self) -> Alg {
        let nodes = self.nodes.iter().rev().map(|m| m.invert()).collect();
        Alg { nodes }
    }

    // pub fn parse(s: impl AsRef<str>) -> Alg {
    //     // fn until_eof(s: &str) -> IResult<&str, &str> {
    //    preceded(multispace0, )
    //     // }
    // }
}

impl PartialEq<Alg> for Alg {
    fn eq(&self, other: &Alg) -> bool {
        self.nodes == other.nodes
    }
}

// macro_rules! alg! {
//     ($a:expr) => {{
//         Alg::parse($a).unwrap()
//     }};
// }
