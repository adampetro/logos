use std::collections::HashSet;
use std::fmt::Debug;

use crate::graph::arena::HasNodeIds;
use crate::graph::{Fork, Graph, NodeId};
use crate::specification::{Sequence, Specification};

#[derive(Debug, Clone)]
pub(crate) struct Rope {
    pub(crate) pattern: Vec<HashSet<u8>>,
    pub(crate) then: NodeId,
    pub(crate) miss: Option<NodeId>,
    pub(crate) record_miss_backtrack_idx: Option<NodeId>,
}

impl Rope {
    pub(crate) fn try_from_sequence(
        sequence: &Sequence,
        then: NodeId,
        miss: Option<NodeId>,
        record_miss_backtrack_idx: Option<NodeId>,
    ) -> Option<Self> {
        sequence
            .iter()
            .map(|specification| match specification {
                Specification::Byte(byte) => Some(HashSet::from([*byte])),
                Specification::Any(any) => any
                    .iter()
                    .map(|specification| specification.as_byte().copied())
                    .collect::<Option<HashSet<u8>>>(),
                _ => None,
            })
            .collect::<Option<Vec<HashSet<u8>>>>()
            .map(|pattern| Self {
                pattern,
                miss,
                then,
                record_miss_backtrack_idx,
            })
    }

    pub(crate) fn fork_off(self, graph: &mut Graph) -> Fork {
        let Self {
            mut pattern,
            then,
            miss,
            record_miss_backtrack_idx,
        } = self;

        let first = pattern.remove(0);

        let then = if pattern.is_empty() {
            then
        } else {
            graph.insert(Self {
                pattern,
                then,
                miss,
                record_miss_backtrack_idx: None,
            })
        };

        let mut fork = Fork::new(miss, record_miss_backtrack_idx);

        first.iter().for_each(|byte| {
            fork.lookup_table[*byte as usize] = Some(then);
        });

        fork
    }
}

impl HasNodeIds for Rope {
    fn update_node_ids(&mut self, f: impl Fn(&mut NodeId)) {
        f(&mut self.then);

        if let Some(miss) = &mut self.miss {
            f(miss);
        }

        if let Some(record_miss_backtrack_idx) = &mut self.record_miss_backtrack_idx {
            f(record_miss_backtrack_idx);
        }
    }
}
