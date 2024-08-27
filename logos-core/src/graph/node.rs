use crate::graph::{arena::HasNodeIds, Fork, NodeId, Rope};
use crate::VariantMatch;
use itertools::Itertools;

#[derive(Debug)]
pub enum Node<'a, T: VariantMatch> {
    Fork(Fork),
    VariantMatch(&'a T),
    Rope(Rope),
}

/// assume variant match is only put in the graph once,
/// this allows us to not require `PartialEq` on `T`
impl<T: VariantMatch> PartialEq for Node<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fork(fork), Self::Fork(other_fork)) => fork == other_fork,
            (Self::Rope(rope), Self::Rope(other_rope)) => rope == other_rope,
            _ => false,
        }
    }
}

impl<T: VariantMatch> From<Fork> for Node<'_, T> {
    fn from(fork: Fork) -> Self {
        if fork.lookup_table.iter().copied().flatten().unique().count() == 1 {
            let pattern = fork
                .lookup_table
                .iter()
                .enumerate()
                .filter_map(|(byte, node_id)| node_id.map(|_| byte as u8))
                .collect();
            let then = fork
                .lookup_table
                .iter()
                .find_map(|node_id| *node_id)
                .unwrap();
            Self::Rope(Rope::new(vec![pattern], then).with_fork_miss(&fork))
        } else {
            Self::Fork(fork)
        }
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

impl<'a, T: VariantMatch> Node<'a, T> {
    pub fn miss(&self) -> Option<NodeId> {
        match self {
            Self::Fork(fork) => fork.miss(),
            Self::VariantMatch(_) => None,
            Self::Rope(rope) => rope.miss(),
        }
    }

    pub fn record_miss_backtrack_idx(&self) -> Option<NodeId> {
        match self {
            Self::Fork(fork) => fork.record_miss_backtrack_idx(),
            Self::VariantMatch(_) => None,
            Self::Rope(rope) => rope.record_miss_backtrack_idx(),
        }
    }
}
