use crate::graph::{arena::HasNodeIds, Fork, NodeId, Rope};
use crate::VariantMatch;

#[derive(Debug, PartialEq)]
pub enum Node<'a, T: VariantMatch> {
    Fork(Fork),
    VariantMatch(&'a T),
    Rope(Rope),
}

impl<T: VariantMatch> From<Fork> for Node<'_, T> {
    fn from(fork: Fork) -> Self {
        Self::Fork(fork)
    }
}

impl<'a, T: VariantMatch> From<&'a T> for Node<'a, T> {
    fn from(variant_match: &'a T) -> Self {
        Self::VariantMatch(variant_match)
    }
}

impl<T: VariantMatch> From<Rope> for Node<'_, T> {
    fn from(rope: Rope) -> Self {
        Self::Rope(rope)
    }
}

impl<T: VariantMatch> HasNodeIds for Node<'_, T> {
    fn update_node_ids(&mut self, f: impl Fn(&mut NodeId)) {
        match self {
            Self::Fork(fork) => fork.update_node_ids(f),
            Self::VariantMatch(_) => {}
            Self::Rope(rope) => rope.update_node_ids(f),
        }
    }
}
