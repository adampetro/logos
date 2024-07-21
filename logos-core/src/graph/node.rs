use crate::graph::{arena::HasNodeIds, Fork, NodeId, Rope, VariantMatch};
use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
pub(crate) enum Node {
    Fork(Fork),
    VariantMatch(VariantMatch),
    Rope(Rope),
}

impl From<Fork> for Node {
    fn from(fork: Fork) -> Self {
        Node::Fork(fork)
    }
}

impl From<VariantMatch> for Node {
    fn from(variant_match: VariantMatch) -> Self {
        Node::VariantMatch(variant_match)
    }
}

impl From<Rope> for Node {
    fn from(rope: Rope) -> Self {
        Node::Rope(rope)
    }
}

impl HasNodeIds for Node {
    fn update_node_ids(&mut self, f: impl Fn(&mut NodeId)) {
        match self {
            Node::Fork(fork) => fork.update_node_ids(f),
            Node::VariantMatch(_) => {}
            Node::Rope(rope) => rope.update_node_ids(f),
        }
    }
}
