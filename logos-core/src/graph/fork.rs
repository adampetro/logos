use std::fmt::Debug;

use crate::graph::arena::HasNodeIds;
use crate::graph::{Graph, Node, NodeId};
use crate::specification::Any;

pub(crate) const LOOKUP_TABLE_SIZE: usize = 256;

#[derive(Clone)]
pub(crate) struct Fork {
    pub(crate) lookup_table: Box<[Option<NodeId>; LOOKUP_TABLE_SIZE]>,
    pub(crate) miss: Option<NodeId>,
    pub(crate) record_miss_backtrack_idx: Option<NodeId>,
}

impl Fork {
    pub(crate) fn new(miss: Option<NodeId>, record_miss_backtrack_idx: Option<NodeId>) -> Self {
        Self {
            lookup_table: Box::new([None; LOOKUP_TABLE_SIZE]),
            miss,
            record_miss_backtrack_idx,
        }
    }

    pub(crate) fn try_from_any(
        any: &Any,
        then: NodeId,
        miss: Option<NodeId>,
        record_miss_backtrack_idx: Option<NodeId>,
    ) -> Option<Self> {
        any.iter()
            .map(|specification| specification.as_byte().copied())
            .collect::<Option<Vec<u8>>>()
            .map(|bytes| {
                let mut fork = Fork::new(miss, record_miss_backtrack_idx);

                bytes.iter().for_each(|byte| {
                    fork.lookup_table[*byte as usize] = Some(then);
                });

                fork
            })
    }

    pub(crate) fn merge<T: Clone>(&mut self, other: Fork, graph: &mut Graph<T>) {
        (0..LOOKUP_TABLE_SIZE).for_each(|idx| {
            let other_to = other.lookup_table[idx];
            let to = self.lookup_table[idx];

            let new_to = match (other_to, to) {
                (None, None) => None,
                (Some(id), None) | (None, Some(id)) => Some(id),
                (Some(from_id), Some(to_id)) => Some(graph.merge(from_id, to_id)),
            };

            self.lookup_table[idx] = new_to;
        });
        if let Some(miss) = other.miss {
            if self.miss.is_none() {
                match &graph[miss] {
                    Node::Fork(fork) => {
                        let fork = fork.clone();
                        self.merge(fork, graph);
                    }
                    Node::Rope(rope) => {
                        let rope = rope.clone();
                        let fork = rope.fork_off(graph);
                        self.merge(fork, graph);
                    }
                    Node::VariantMatch(_) => {
                        self.miss = Some(miss);
                        let mut visited = Default::default();
                        (0..LOOKUP_TABLE_SIZE).for_each(|node_id| {
                            if let Some(node_id) = self.lookup_table[node_id] {
                                graph.propagate_miss(node_id, miss, &mut visited);
                            }
                        });
                    }
                }
            }
        }
        if let Some(record_miss_backtrack_idx) = other.record_miss_backtrack_idx {
            self.record_miss_backtrack_idx
                .get_or_insert(record_miss_backtrack_idx);
        }
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
