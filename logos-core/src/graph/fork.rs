use std::fmt::Debug;

use crate::graph::{arena::HasNodeIds, GraphBuilder, Node, NodeId};
use crate::specification::Any;
use crate::VariantMatch;

pub const LOOKUP_TABLE_SIZE: usize = 256;

#[derive(Clone, PartialEq)]
pub struct Fork {
    pub(crate) lookup_table: Box<[Option<NodeId>; LOOKUP_TABLE_SIZE]>,
    miss: Option<NodeId>,
    record_miss_backtrack_idx: Option<NodeId>,
}

impl Fork {
    pub(crate) fn new() -> Self {
        Self {
            lookup_table: Box::new([None; LOOKUP_TABLE_SIZE]),
            miss: None,
            record_miss_backtrack_idx: None,
        }
    }

    pub(crate) fn try_from_any(any: &Any, then: NodeId) -> Option<Self> {
        any.iter()
            .map(|specification| specification.as_byte().copied())
            .collect::<Option<Vec<u8>>>()
            .map(|bytes| {
                let mut fork = Fork::new();

                bytes.iter().for_each(|byte| {
                    fork.lookup_table[*byte as usize] = Some(then);
                });

                fork
            })
    }

    pub(crate) fn with_miss<T: VariantMatch>(
        mut self,
        node_id_and_record_miss_backtrack_idx: impl Into<Option<(NodeId, bool)>>,
        graph_builder: &mut GraphBuilder<T>,
    ) -> Self {
        let Some((node_id, record_miss_backtrack_idx)) =
            node_id_and_record_miss_backtrack_idx.into()
        else {
            return self;
        };
        self.miss = Some(node_id);
        self.record_miss_backtrack_idx = record_miss_backtrack_idx.then_some(node_id);
        while let Some(miss) = self.miss {
            match &graph_builder[miss] {
                Some(Node::Fork(fork)) => {
                    self.miss = None;
                    self.record_miss_backtrack_idx = None;
                    self.merge(fork.clone(), graph_builder);
                }
                Some(Node::Rope(rope)) => {
                    self.miss = None;
                    self.record_miss_backtrack_idx = None;
                    self.merge(rope.clone().fork_off(graph_builder), graph_builder);
                }
                Some(Node::VariantMatch(_)) | None => break,
            }
        }
        self
    }

    pub(crate) fn with_miss_unchecked(
        mut self,
        miss: Option<NodeId>,
        record_miss_backtrack_idx: Option<NodeId>,
    ) -> Self {
        self.miss = miss;
        self.record_miss_backtrack_idx = record_miss_backtrack_idx;
        self
    }

    pub(crate) fn merge<T: VariantMatch>(
        &mut self,
        other: Fork,
        graph_builder: &mut GraphBuilder<T>,
    ) {
        self.miss = match (self.miss, other.miss) {
            (Some(self_miss), Some(other_miss)) => Some(graph_builder.merge(self_miss, other_miss)),
            (None, Some(other_miss)) => Some(other_miss),
            (self_miss, None) => self_miss,
        };
        self.record_miss_backtrack_idx = match (
            self.record_miss_backtrack_idx,
            other.record_miss_backtrack_idx,
        ) {
            (Some(self_node_id), Some(other_node_id)) => {
                Some(graph_builder.merge(self_node_id, other_node_id))
            }
            (None, Some(other_node_id)) => Some(other_node_id),
            (self_node_id, None) => self_node_id,
        };
        let miss_fork_id = self.miss.map(|miss| {
            let miss_fork = Fork::new().with_miss(Some((miss, false)), graph_builder);
            graph_builder.insert(miss_fork)
        });
        self.lookup_table
            .iter_mut()
            .zip(other.lookup_table.iter())
            .for_each(|(to, other_to)| {
                let new_to = match (*other_to, *to) {
                    (None, None) => None,
                    (Some(id), None) => Some(id),
                    (None, Some(id)) => Some(id),
                    (Some(self_id), Some(other_id)) => Some(graph_builder.merge(self_id, other_id)),
                }
                .map(|new_to| {
                    if let Some(miss_fork_id) = miss_fork_id {
                        let merge_id = graph_builder.merge(miss_fork_id, new_to);
                        match &graph_builder[merge_id] {
                            Some(Node::Fork(fork)) => fork.flatten_to_miss().unwrap_or(merge_id),
                            _ => merge_id,
                        }
                    } else {
                        new_to
                    }
                });

                *to = new_to;
            });
    }

    fn flatten_to_miss(&self) -> Option<NodeId> {
        match self.miss {
            Some(miss)
                if self.lookup_table_is_empty() && self.record_miss_backtrack_idx == self.miss =>
            {
                Some(miss)
            }
            _ => None,
        }
    }

    fn lookup_table_is_empty(&self) -> bool {
        self.lookup_table.iter().all(Option::is_none)
    }

    pub fn lookup_table(&self) -> &[Option<NodeId>; 256] {
        &self.lookup_table
    }

    pub fn miss(&self) -> Option<NodeId> {
        self.miss
    }

    pub fn record_miss_backtrack_idx(&self) -> Option<NodeId> {
        self.record_miss_backtrack_idx
    }
}

impl Debug for Fork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lookup_table = self
            .lookup_table
            .iter()
            .enumerate()
            .filter_map(|(byte, node_id)| node_id.map(|node_id| (byte, node_id)))
            .collect::<Vec<_>>();
        f.debug_struct("Fork")
            .field("lookup_table", &lookup_table)
            .field("miss", &self.miss)
            .field("record_miss_backtrack_idx", &self.record_miss_backtrack_idx)
            .finish()
    }
}

impl HasNodeIds for Fork {
    fn update_node_ids(&mut self, f: impl Fn(&mut NodeId)) {
        self.lookup_table.iter_mut().for_each(|node_id| {
            if let Some(node_id) = node_id {
                f(node_id);
            }
        });
        if let Some(miss) = self.miss.as_mut() {
            f(miss);
        }
        if let Some(record_miss_backtrack_idx) = self.record_miss_backtrack_idx.as_mut() {
            f(record_miss_backtrack_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Fork, GraphBuilder};
    use crate::SimpleVariantMatch;

    #[test]
    fn test_simple_merge() {
        let mut graph_builder = GraphBuilder::default();
        let leaf1 = SimpleVariantMatch::new("a", 2);
        let leaf2 = SimpleVariantMatch::new("b", 2);
        let leaf1_id = graph_builder.insert(&leaf1);
        let leaf2_id = graph_builder.insert(&leaf2);
        let mut fork1 = Fork::new();
        fork1.lookup_table[b'a' as usize] = Some(leaf1_id);
        let mut fork2 = Fork::new();
        fork2.lookup_table[b'b' as usize] = Some(leaf2_id);
        fork1.merge(fork2, &mut graph_builder);
        assert_eq!(fork1.lookup_table[b'a' as usize], Some(leaf1_id));
        assert_eq!(fork1.lookup_table[b'b' as usize], Some(leaf2_id));
    }

    #[test]
    fn with_miss_merges_fork() {
        let mut graph_builder = GraphBuilder::default();
        let leaf1 = SimpleVariantMatch::new("a", 2);
        let leaf2 = SimpleVariantMatch::new("b", 2);
        let leaf3 = SimpleVariantMatch::new("c", 2);
        let leaf1_id = graph_builder.insert(&leaf1);
        let leaf2_id = graph_builder.insert(&leaf2);
        let leaf3_id = graph_builder.insert(&leaf3);
        let mut fork1 = Fork::new();
        fork1.lookup_table[b'a' as usize] = Some(leaf1_id);
        let mut fork2 = Fork::new();
        fork2.lookup_table[b'b' as usize] = Some(leaf2_id);
        let mut fork3 = Fork::new();
        fork3.lookup_table[b'c' as usize] = Some(leaf3_id);
        let fork3_id = graph_builder.insert(fork3);
        fork2 = fork2.with_miss(Some((fork3_id, true)), &mut graph_builder);
        let fork2_id = graph_builder.insert(fork2);
        fork1 = fork1.with_miss(Some((fork2_id, true)), &mut graph_builder);
        assert_eq!(fork1.lookup_table[b'a' as usize], Some(leaf1_id));
        assert_eq!(fork1.lookup_table[b'b' as usize], Some(leaf2_id));
        assert_eq!(fork1.lookup_table[b'c' as usize], Some(leaf3_id));
        assert_eq!(fork1.miss(), None);
        assert_eq!(fork1.record_miss_backtrack_idx(), None);
    }
}
