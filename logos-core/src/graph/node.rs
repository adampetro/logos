use crate::graph::{arena::HasNodeIds, Fork, NodeId, Rope};
use crate::VariantMatch;

#[derive(Debug)]
pub enum Node<'a, T: VariantMatch> {
    Fork(Fork),
    VariantMatch(&'a T),
    Rope(Rope),
}

impl<T: VariantMatch> PartialEq for Node<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Fork(fork), Node::Fork(other_fork)) => fork == other_fork,
            (Node::VariantMatch(variant_match), Node::VariantMatch(other_variant_match)) => {
                variant_match.is_same_variant(other_variant_match)
                    && variant_match.priority() == other_variant_match.priority()
                    && variant_match.specification() == other_variant_match.specification()
            }
            (Node::Rope(rope), Node::Rope(other_rope)) => rope == other_rope,
            _ => false,
        }
    }
}

impl<T: VariantMatch> From<Fork> for Node<'_, T> {
    fn from(fork: Fork) -> Self {
        Node::Fork(fork)
    }
}

impl<'a, T: VariantMatch> From<&'a T> for Node<'a, T> {
    fn from(variant_match: &'a T) -> Self {
        Node::VariantMatch(variant_match)
    }
}

impl<T: VariantMatch> From<Rope> for Node<'_, T> {
    fn from(rope: Rope) -> Self {
        Node::Rope(rope)
    }
}

impl<T: VariantMatch> HasNodeIds for Node<'_, T> {
    fn update_node_ids(&mut self, f: impl Fn(&mut NodeId)) {
        match self {
            Node::Fork(fork) => fork.update_node_ids(f),
            Node::VariantMatch(_) => {}
            Node::Rope(rope) => rope.update_node_ids(f),
        }
    }
}
