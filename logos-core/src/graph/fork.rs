use std::fmt::Debug;

use crate::graph::arena::HasNodeIds;
use crate::graph::{Graph, NodeId};
use crate::specification::Any;

pub(crate) const LOOKUP_TABLE_SIZE: usize = 256;

pub(crate) struct Fork {
    pub(crate) lookup_table: Box<[Option<NodeId>; LOOKUP_TABLE_SIZE]>,
    pub(crate) miss: Option<NodeId>,
}

impl Fork {
    pub(crate) fn try_from_any(any: &Any, then: NodeId, miss: Option<NodeId>) -> Option<Self> {
        any.iter()
            .map(|specification| specification.as_byte().copied())
            .collect::<Option<Vec<u8>>>()
            .map(|bytes| {
                let mut fork = Fork::default().with_miss(miss);

                bytes.iter().for_each(|byte| {
                    fork.lookup_table[*byte as usize] = Some(then);
                });

                fork
            })
    }

    pub(crate) fn merge(&mut self, other: Fork, graph: &mut Graph) {
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
    }

    pub(crate) fn with_miss(mut self, miss: impl Into<Option<NodeId>>) -> Self {
        self.miss = miss.into();
        self
    }
}

impl Default for Fork {
    fn default() -> Self {
        Fork {
            lookup_table: Box::new([None; 256]),
            miss: None,
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
    }
}
