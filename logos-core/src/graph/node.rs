use crate::graph::{arena::HasNodeIds, Fork, NodeId, Rope, VariantMatch};
use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
pub enum Node<T> {
    Fork(Fork),
    VariantMatch(VariantMatch<T>),
    Rope(Rope),
}

impl<T> From<Fork> for Node<T> {
    fn from(fork: Fork) -> Self {
        Node::Fork(fork)
    }
}

impl<T> From<VariantMatch<T>> for Node<T> {
    fn from(variant_match: VariantMatch<T>) -> Self {
        Node::VariantMatch(variant_match)
    }
}

impl<T> From<Rope> for Node<T> {
    fn from(rope: Rope) -> Self {
        Node::Rope(rope)
    }
}

impl<T> HasNodeIds for Node<T> {
    fn update_node_ids(&mut self, f: impl Fn(&mut NodeId)) {
        match self {
            Node::Fork(fork) => fork.update_node_ids(f),
            Node::VariantMatch(_) => {}
            Node::Rope(rope) => rope.update_node_ids(f),
        }
    }
}
