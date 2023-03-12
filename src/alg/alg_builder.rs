use super::{Alg, AlgNode};

pub enum AlgFragment {
    Alg(Alg),
    AlgNode(AlgNode),
}

impl From<&Alg> for AlgFragment {
    fn from(alg: &Alg) -> AlgFragment {
        AlgFragment::Alg(alg.clone())
    }
}

impl From<&AlgNode> for AlgFragment {
    fn from(alg_node: &AlgNode) -> AlgFragment {
        AlgFragment::AlgNode(alg_node.clone())
    }
}

pub struct AlgBuilder {
    fragments: Vec<AlgFragment>,
}

impl AlgBuilder {
    pub fn new(fragment: Option<impl Into<AlgFragment>>) -> AlgBuilder {
        let fragments = match fragment {
            Some(fragment) => vec![fragment.into()],
            None => vec![],
        };
        AlgBuilder { fragments }
    }

    pub fn push(&mut self, fragment: impl Into<AlgFragment>) {
        self.fragments.push(fragment.into());
    }

    // pub fn prepend(&mut self, fragment: impl Into<AlgFragment>) {
    //     self.fragments.insert(0, fragment.into());
    // }

    pub fn to_alg(mut self) -> Alg {
        // TODO: use iterators to do this more efficiently.
        let drained = self.fragments.drain(0..); // TODO: `into_iter`?
        let mapped = drained.flat_map(|fragment| match fragment {
            AlgFragment::Alg(alg) => alg.nodes.into_iter(),
            // TODO: avoid constructing `vec!`.
            AlgFragment::AlgNode(alg_node) => vec![alg_node].into_iter(),
        });
        let nodes = mapped.collect();
        Alg { nodes }
    }
}

impl Default for AlgBuilder {
    fn default() -> Self {
        Self::new(None::<AlgFragment>)
    }
}
