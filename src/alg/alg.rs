use super::Move;

pub struct Alg {
    pub nodes: Vec<Move>,
}

impl Alg {
    pub fn invert(&self) -> Alg {
        let nodes = self.nodes.iter().rev().map(|m| m.invert()).collect();
        Alg { nodes }
    }
}

impl PartialEq<Alg> for Alg {
    fn eq(&self, other: &Alg) -> bool {
        self.nodes == other.nodes
    }
}
