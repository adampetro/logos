use std::collections::HashSet;
use std::fmt::Debug;

use crate::graph::{arena::HasNodeIds, Fork, GraphBuilder, Node, NodeId};
use crate::specification::{Sequence, Specification};
use crate::VariantMatch;

#[derive(Debug, Clone, PartialEq)]
pub struct Rope {
    pub(crate) pattern: Vec<HashSet<u8>>,
    pub(crate) then: NodeId,
    miss: Option<NodeId>,
    record_miss_backtrack_idx: Option<NodeId>,
}

impl Rope {
    pub(crate) fn new(pattern: Vec<HashSet<u8>>, then: NodeId) -> Self {
        Self {
            pattern,
            then,
            miss: None,
            record_miss_backtrack_idx: None,
        }
    }

    pub(crate) fn try_from_sequence(sequence: &Sequence, then: NodeId) -> Option<Self> {
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
            .map(|pattern| Self::new(pattern, then))
    }

    pub(crate) fn fork_off<T: VariantMatch>(self, graph_builder: &mut GraphBuilder<T>) -> Fork {
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
            graph_builder.insert(Self {
                pattern,
                then,
                miss,
                record_miss_backtrack_idx: None,
            })
        };

        let mut fork = Fork::new().with_miss_unchecked(miss, record_miss_backtrack_idx);

        first.iter().for_each(|byte| {
            fork.lookup_table[*byte as usize] = Some(then);
        });

        fork
    }

    pub(crate) fn with_miss<'a, T: VariantMatch>(
        mut self,
        node_id_and_record_miss_backtrack_idx: impl Into<Option<(NodeId, bool)>>,
        graph_builder: &mut GraphBuilder<T>,
    ) -> Node<'a, T> {
        let Some((node_id, record_miss_backtrack_idx)) =
            node_id_and_record_miss_backtrack_idx.into()
        else {
            return self.into();
        };
        if matches!(&graph_builder[node_id], Some(Node::VariantMatch(_))) {
            self.miss = Some(node_id);
            self.record_miss_backtrack_idx = record_miss_backtrack_idx.then_some(node_id);
            self.into()
        } else {
            self.fork_off(graph_builder)
                .with_miss(Some((node_id, record_miss_backtrack_idx)), graph_builder)
                .into()
        }
    }

    pub(crate) fn with_fork_miss(mut self, fork: &Fork) -> Self {
        self.miss = fork.miss();
        self.record_miss_backtrack_idx = fork.record_miss_backtrack_idx();
        self
    }

    pub fn pattern(&self) -> &[HashSet<u8>] {
        &self.pattern
    }

    pub fn miss(&self) -> Option<NodeId> {
        self.miss
    }

    pub fn then(&self) -> NodeId {
        self.then
    }

    pub fn record_miss_backtrack_idx(&self) -> Option<NodeId> {
        self.record_miss_backtrack_idx
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
