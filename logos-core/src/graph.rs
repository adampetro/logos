mod arena;
mod fork;
mod node;
mod rope;

use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use crate::{Lexer, Specification, VariantMatch};
use arena::Arena;
pub use arena::NodeId;
pub use fork::Fork;
pub use node::Node;
pub use rope::Rope;

#[derive(Debug)]
struct ReservedId(NodeId);

#[derive(Debug, PartialEq, Eq, Hash)]
struct MergeKey(NodeId, NodeId);

impl MergeKey {
    fn new(a: NodeId, b: NodeId) -> Self {
        if a < b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

#[derive(Debug)]
struct DeferredMerge {
    awaiting: NodeId,
    with: NodeId,
    into: ReservedId,
}

#[derive(Debug, PartialEq)]
pub enum Error<'a, T: VariantMatch> {
    VariantMatchesOverlapWithSamePriority(&'a T, &'a T),
}

#[derive(Debug)]
pub struct Graph<'a, T: VariantMatch> {
    nodes: Arena<Node<'a, T>>,
    start_node_id: NodeId,
}

impl<'a, T: VariantMatch> Graph<'a, T> {
    pub fn start_node_id(&self) -> NodeId {
        self.start_node_id
    }

    pub fn for_lexer(lexer: &'a Lexer<T>) -> Result<Self, Vec<Error<'a, T>>> {
        let mut builder = GraphBuilder::default();
        let mut start_fork = Fork::new();

        lexer.variant_matches().iter().for_each(|variant_match| {
            let fork_for_variant_match = builder.fork_for_variant_match(variant_match);
            start_fork.merge(fork_for_variant_match, &mut builder);
        });

        let start_node_id = builder.insert(start_fork);

        if !builder.errors.is_empty() {
            return Err(builder.errors);
        }

        let start_node_id = builder.shake(start_node_id);

        Ok(Self {
            nodes: builder.nodes.map(|node| node.expect("reserved node")),
            start_node_id,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &Node<T>)> {
        self.nodes.iter()
    }
}

impl<'a, T: VariantMatch> Index<NodeId> for Graph<'a, T> {
    type Output = Node<'a, T>;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index]
    }
}

#[derive(Debug)]
pub(crate) struct GraphBuilder<'a, T: VariantMatch> {
    nodes: Arena<Option<Node<'a, T>>>,
    merges: HashMap<MergeKey, NodeId>,
    errors: Vec<Error<'a, T>>,
    deferred_merges: Vec<DeferredMerge>,
}

impl<'a, T: VariantMatch> Default for GraphBuilder<'a, T> {
    fn default() -> Self {
        Self {
            nodes: Arena::default(),
            merges: HashMap::new(),
            errors: Vec::new(),
            deferred_merges: Vec::new(),
        }
    }
}

impl<'a, T: VariantMatch> GraphBuilder<'a, T> {
    fn insert(&mut self, node: impl Into<Node<'a, T>>) -> NodeId {
        let node = node.into();

        let existing_node_id = self
            .nodes
            .iter()
            .find(|(_, node_for_id)| matches!(node_for_id, Some(n) if n == &node))
            .map(|(id, _)| id);

        if let Some(existing_node_id) = existing_node_id {
            existing_node_id
        } else {
            self.nodes.insert(Some(node))
        }
    }

    fn insert_reserved(&mut self, reserved_id: ReservedId, node: impl Into<Node<'a, T>>) -> NodeId {
        self.nodes[reserved_id.0] = Some(node.into());

        let (related, unrelated) = self
            .deferred_merges
            .drain(..)
            .partition::<Vec<_>, _>(|merge| merge.awaiting == reserved_id.0);

        self.deferred_merges = unrelated;

        related.into_iter().for_each(
            |DeferredMerge {
                 awaiting,
                 with,
                 into,
             }| {
                self.merge_unchecked(awaiting, with, into);
            },
        );

        reserved_id.0
    }

    fn fork_for_variant_match(
        &mut self,
        (specification, variant_match): &'a (Specification, T),
    ) -> Fork {
        let terminal = self.insert(variant_match);

        let node = self.node_for_specification(specification, terminal, None);

        match node {
            Node::Fork(fork) => fork,
            Node::Rope(rope) => {
                let mut fork = Fork::new();
                let f = rope.fork_off(self);
                fork.merge(f, self);
                fork
            }
            Node::VariantMatch(_) => unreachable!("variant match implies an empty specification"),
        }
    }

    fn node_for_specification(
        &mut self,
        specification: &Specification,
        then: NodeId,
        miss: Option<(NodeId, bool)>,
    ) -> Node<'a, T> {
        match specification {
            Specification::Byte(value) => {
                Rope::new(vec![HashSet::from([*value])], then).with_miss(miss, self)
            }
            Specification::Any(any) => Fork::try_from_any(any, then)
                .map(|fork| Node::from(fork.with_miss(miss, self)))
                .unwrap_or_else(|| {
                    let mut fork = Fork::new().with_miss(miss, self);

                    any.iter().for_each(|specification| {
                        let node = self.node_for_specification(specification, then, miss);
                        match node {
                            Node::Fork(f) => fork.merge(f, self),
                            Node::Rope(rope) => {
                                let f = rope.fork_off(self);
                                fork.merge(f, self);
                            }
                            Node::VariantMatch(_) => {
                                unreachable!("variant match implies an empty specification")
                            }
                        }
                    });

                    fork.into()
                }),
            Specification::Sequence(sequence) => Rope::try_from_sequence(sequence, then)
                .map(|rope| rope.with_miss(miss, self))
                .unwrap_or_else(|| {
                    let sequence_length = sequence.len();
                    let mut reverse_iterator = sequence
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(idx, specification)| ((idx + 1 == sequence_length), specification));
                    let (is_last, specification) = reverse_iterator.next().unwrap();
                    let then_node = self.node_for_specification(
                        specification,
                        then,
                        miss.map(|(node_id, record_miss_backtrack_idx)| {
                            (node_id, record_miss_backtrack_idx && is_last)
                        }),
                    );
                    reverse_iterator.fold(then_node, |then_node, (is_last, specification)| {
                        let then = self.insert(then_node);
                        self.node_for_specification(
                            specification,
                            then,
                            miss.map(|(node_id, record_miss_backtrack_idx)| {
                                (node_id, record_miss_backtrack_idx && is_last)
                            }),
                        )
                    })
                }),
            Specification::Loop(l) => {
                if let Some(max) = l.max() {
                    let terminal = then;
                    let min = l.min();

                    let last_miss = if min == max {
                        miss.map(|(node_id, _)| (node_id, false))
                    } else {
                        Some((terminal, true))
                    };

                    let then_node =
                        self.node_for_specification(l.specification(), terminal, last_miss);

                    let then_node = (min..(max - 1)).fold(then_node, |then_node, _| {
                        let then = self.insert(then_node);
                        self.node_for_specification(l.specification(), then, Some((terminal, true)))
                    });

                    (0..min).fold(then_node, |then_node, i| {
                        let then = self.insert(then_node);
                        let is_last = i + 1 == min;
                        self.node_for_specification(
                            l.specification(),
                            then,
                            miss.map(|(node_id, record_miss_backtrack_idx)| {
                                (node_id, record_miss_backtrack_idx && is_last)
                            }),
                        )
                    })
                } else {
                    let reserved_loop_back_to = self.reserve();

                    let loop_end = self.node_for_specification(
                        l.specification(),
                        reserved_loop_back_to.0,
                        Some((then, true)),
                    );

                    let start_id = self.insert_reserved(reserved_loop_back_to, loop_end);

                    let then_node = self.node_for_specification(
                        l.specification(),
                        start_id,
                        Some((then, true)),
                    );

                    (0..l.min()).fold(then_node, |then_node, i| {
                        let then = self.insert(then_node);
                        let is_last = i + 1 == l.min();
                        self.node_for_specification(
                            l.specification(),
                            then,
                            miss.map(|(node_id, record_miss_backtrack_idx)| {
                                (node_id, record_miss_backtrack_idx && is_last)
                            }),
                        )
                    })
                }
            }
        }
    }

    fn shake(&mut self, start_node_id: NodeId) -> NodeId {
        let mut seen_nodes = HashSet::new();
        self.visit_node(start_node_id, &mut seen_nodes);
        let node_ids_to_delete = self
            .nodes
            .iter_ids()
            .filter(|node_id| !seen_nodes.contains(node_id));
        self.nodes.delete_nodes(node_ids_to_delete, start_node_id)
    }

    fn visit_node(&self, node_id: NodeId, seen_nodes: &mut HashSet<NodeId>) {
        if !seen_nodes.insert(node_id) {
            return;
        }
        match &self.nodes[node_id] {
            Some(Node::Fork(fork)) => {
                fork.lookup_table.iter().for_each(|node_id| {
                    if let Some(node_id) = node_id {
                        self.visit_node(*node_id, seen_nodes);
                    }
                });
                if let Some(miss) = fork.miss() {
                    self.visit_node(miss, seen_nodes);
                }
                if let Some(record_miss_backtrack_idx) = fork.record_miss_backtrack_idx() {
                    self.visit_node(record_miss_backtrack_idx, seen_nodes);
                }
            }
            Some(Node::VariantMatch(_)) | None => {}
            Some(Node::Rope(rope)) => {
                self.visit_node(rope.then, seen_nodes);
                if let Some(miss) = rope.miss() {
                    self.visit_node(miss, seen_nodes);
                }
                if let Some(record_miss_backtrack_idx) = rope.record_miss_backtrack_idx() {
                    self.visit_node(record_miss_backtrack_idx, seen_nodes);
                }
            }
        }
    }

    fn fork_off(&mut self, node_id: NodeId) -> Fork {
        match &self.nodes[node_id] {
            Some(Node::Fork(fork)) => fork.clone(),
            Some(Node::Rope(rope)) => rope.clone().fork_off(self),
            Some(Node::VariantMatch(_)) | None => {
                Fork::new().with_miss(Some((node_id, true)), self)
            }
        }
    }

    pub(crate) fn merge(&mut self, a: NodeId, b: NodeId) -> NodeId {
        if a == b {
            return a;
        }

        let key = MergeKey::new(a, b);
        if let Some(node_id) = self.merges.get(&key) {
            return *node_id;
        }

        let node_a = &self.nodes[a];
        let node_b = &self.nodes[b];

        match (node_a, node_b) {
            (
                Some(Node::VariantMatch(variant_match_a)),
                Some(Node::VariantMatch(variant_match_b)),
            ) => {
                let merge_id = if variant_match_a.priority() == variant_match_b.priority() {
                    self.errors
                        .push(Error::VariantMatchesOverlapWithSamePriority(
                            variant_match_a,
                            variant_match_b,
                        ));
                    b
                } else if variant_match_a.priority() > variant_match_b.priority() {
                    a
                } else {
                    b
                };
                self.set_merged(a, b, merge_id);

                merge_id
            }
            (None, None) => {
                panic!(
                    "Merging two reserved nodes! This is a bug, please report it:\n\
                    \n\
                    https://github.com/maciejhirsz/logos/issues"
                );
            }
            (None, Some(_)) => {
                let reserved = self.reserve();
                let merge_id = reserved.0;
                self.set_merged(a, b, merge_id);
                self.deferred_merges.push(DeferredMerge {
                    awaiting: a,
                    with: b,
                    into: reserved,
                });
                merge_id
            }
            (Some(_), None) => {
                let reserved = self.reserve();
                let merge_id = reserved.0;
                self.set_merged(a, b, merge_id);
                self.deferred_merges.push(DeferredMerge {
                    awaiting: b,
                    with: a,
                    into: reserved,
                });
                merge_id
            }
            (Some(_), Some(_)) => {
                let reserved = self.reserve();
                let merge_id = reserved.0;
                self.set_merged(a, b, merge_id);
                self.merge_unchecked(a, b, reserved)
            }
        }
    }

    fn merge_unchecked(&mut self, a: NodeId, b: NodeId, reserved: ReservedId) -> NodeId {
        let (Some(_), Some(_)) = (&self.nodes[a], &self.nodes[b]) else {
            panic!(
                "Merging unchecked with one or more reserved nodes! This is a bug, please report it:\n\
                \n\
                https://github.com/maciejhirsz/logos/issues"
            );
        };

        let fork_a = self.fork_off(a);
        let mut fork_b = self.fork_off(b);
        fork_b.merge(fork_a, self);
        self.insert_reserved(reserved, fork_b)
    }

    fn set_merged(&mut self, a: NodeId, b: NodeId, merge_id: NodeId) {
        self.merges.insert(MergeKey::new(a, b), merge_id);
        self.merges.insert(MergeKey::new(b, merge_id), merge_id);
        self.merges.insert(MergeKey::new(a, merge_id), merge_id);
    }

    fn reserve(&mut self) -> ReservedId {
        ReservedId(self.nodes.insert(None))
    }
}

impl<'a, T: VariantMatch> Index<NodeId> for GraphBuilder<'a, T> {
    type Output = Option<Node<'a, T>>;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index]
    }
}

#[cfg(test)]
mod tests {
    use super::{Fork, GraphBuilder, Node, Rope};
    use crate::SimpleVariantMatch;
    use std::collections::HashSet;

    #[test]
    fn test_record_miss_backtrack_idx_properly_propagated_on_fork_rope_merge() {
        let mut graph_builder = GraphBuilder::default();
        let variant_match_a = SimpleVariantMatch::new("a", 2);
        let variant_match_a_id = graph_builder.insert(&variant_match_a);
        let mut fork = Fork::new();
        fork.lookup_table[b'a' as usize] = Some(variant_match_a_id);
        let fork_id = graph_builder.insert(fork);
        let variant_match_abc = SimpleVariantMatch::new("abc", 6);
        let variant_match_abc_id = graph_builder.insert(&variant_match_abc);
        let rope = Rope::new(
            vec![[b'a'].into(), [b'b'].into(), [b'c'].into()],
            variant_match_abc_id,
        );
        let rope_id = graph_builder.insert(rope);
        let merged_id = graph_builder.merge(fork_id, rope_id);
        let merged = &graph_builder[merged_id];
        let Some(Node::Rope(rope)) = merged else {
            panic!("Expected merged node to be a rope, got {:?}", merged);
        };
        assert_eq!(rope.record_miss_backtrack_idx(), None);
        assert_eq!(rope.miss(), None);
        assert_eq!(rope.pattern(), vec![HashSet::from([b'a'])]);
        let node_id = rope.then();
        let node = &graph_builder[node_id];
        let Some(Node::Rope(rope)) = node else {
            panic!("Expected rope to go to another rope, got {:?}", node);
        };
        dbg!(&graph_builder);
        assert_eq!(rope.record_miss_backtrack_idx(), Some(variant_match_a_id));
        assert_eq!(rope.miss(), Some(variant_match_a_id));
        assert_eq!(rope.pattern(), vec![HashSet::from([b'b'])]);
        let node_id = rope.then();
        let node = &graph_builder[node_id];
        let Some(Node::Rope(rope)) = node else {
            panic!("Expected rope to go to another rope, got {:?}", node);
        };
        assert_eq!(rope.record_miss_backtrack_idx(), None);
        assert_eq!(rope.miss(), Some(variant_match_a_id));
    }

    #[test]
    fn test_longer_match_lower_priority() {
        let mut graph_builder = GraphBuilder::default();
        let variant_match_abc = SimpleVariantMatch::new("abc", 6);
        let variant_match_abc_id = graph_builder.insert(&variant_match_abc);
        let rope_def = Rope::new(
            vec![[b'd'].into(), [b'e'].into(), [b'f'].into()],
            variant_match_abc_id,
        )
        .with_miss(Some((variant_match_abc_id, true)), &mut graph_builder);
        let rope_def_id = graph_builder.insert(rope_def);
        let rope_abc = Rope::new(
            vec![[b'a'].into(), [b'b'].into(), [b'c'].into()],
            rope_def_id,
        );
        let rope_abc_id = graph_builder.insert(rope_abc);
        let variant_match_word = SimpleVariantMatch::new("word", 2);
        let variant_match_word_id = graph_builder.insert(&variant_match_word);
        let text_rope_reserved_id = graph_builder.reserve();
        let text_rope = Rope::new(
            vec![(b'a'..=b'z').collect::<HashSet<_>>()],
            text_rope_reserved_id.0,
        )
        .with_miss(Some((variant_match_word_id, true)), &mut graph_builder);
        let text_rope_id = graph_builder.insert_reserved(text_rope_reserved_id, text_rope);
        let text_start_rope = Rope::new(vec![(b'a'..=b'z').collect::<HashSet<_>>()], text_rope_id);
        let text_start_rope_id = graph_builder.insert(text_start_rope);
        let merge_id = graph_builder.merge(rope_abc_id, text_start_rope_id);
        let node = &graph_builder[merge_id];
        let Some(Node::Fork(fork)) = node else {
            panic!("Expected merged node to be a fork, got {:?}", node);
        };
        let node_id = fork.lookup_table[b'a' as usize].unwrap();
        let node = &graph_builder[node_id];
        let Some(Node::Fork(fork)) = node else {
            panic!("Expected merged node to be a fork, got {:?}", node);
        };
        assert_eq!(fork.miss(), Some(variant_match_word_id));
        assert_eq!(fork.record_miss_backtrack_idx(), Some(variant_match_word_id));
        let node_id = fork.lookup_table[b'b' as usize].unwrap();
        let node = &graph_builder[node_id];
        let Some(Node::Fork(fork)) = node else {
            panic!("Expected merged node to be a fork, got {:?}", node);
        };
        assert_eq!(fork.miss(), Some(variant_match_word_id));
        assert_eq!(fork.record_miss_backtrack_idx(), Some(variant_match_word_id));
        let node_id = fork.lookup_table[b'c' as usize].unwrap();
        let node = &graph_builder[node_id];
        let Some(Node::Fork(fork)) = node else {
            panic!("Expected merged node to be a fork, got {:?}", node);
        };
        assert_eq!(fork.miss(), Some(variant_match_abc_id));
        assert_eq!(fork.record_miss_backtrack_idx(), Some(variant_match_abc_id));
        let node_id = fork.lookup_table[b'd' as usize].unwrap();
        let node = &graph_builder[node_id];
        let Some(Node::Fork(fork)) = node else {
            panic!("Expected merged node to be a fork, got {:?}", node);
        };
        dbg!(&graph_builder);
        assert_eq!(fork.miss(), Some(variant_match_word_id));
        assert_eq!(fork.record_miss_backtrack_idx(), Some(variant_match_word_id));
    }

    #[test]
    fn test_overlapping_infinite_loops() {
        let mut graph_builder = GraphBuilder::default();
        let variant_match_abc = SimpleVariantMatch::new("abc", 6);
        let variant_match_abc_id = graph_builder.insert(&variant_match_abc);
        let rope_abc_reserved_id = graph_builder.reserve();
        let rope_abc = Rope::new(
            vec![[b'a'].into(), [b'b'].into(), [b'c'].into()],
            rope_abc_reserved_id.0,
        ).with_miss(Some((variant_match_abc_id, true)), &mut graph_builder);
        let rope_abc_id = graph_builder.insert_reserved(rope_abc_reserved_id, rope_abc);
        let rope_abc_start = Rope::new(
            vec![[b'a'].into(), [b'b'].into(), [b'c'].into()],
            rope_abc_id,
        );
        let rope_abc_start_id = graph_builder.insert(rope_abc_start);
        let variant_match_abcd = SimpleVariantMatch::new("abcd", 8);
        let variant_match_abcd_id = graph_builder.insert(&variant_match_abcd);
        let rope_d = Rope::new(vec![[b'd'].into()], variant_match_abcd_id);
        let rope_d_id = graph_builder.insert(rope_d);
        let rope_abcd_loop_reserved_id = graph_builder.reserve();
        let rope_abcd_loop = Rope::new(
            vec![[b'a'].into(), [b'b'].into(), [b'c'].into()],
            rope_abcd_loop_reserved_id.0,
        ).with_miss(Some((variant_match_abcd_id, true)), &mut graph_builder);
        let rope_abcd_loop_id = graph_builder.insert_reserved(rope_abcd_loop_reserved_id, rope_abcd_loop);
        let rope_abcd_start = Rope::new(
            vec![[b'a'].into(), [b'b'].into(), [b'c'].into()],
            rope_abcd_loop_id,
        );
        let rope_abcd_start_id = graph_builder.insert(rope_abcd_start);
        let merge_id = graph_builder.merge(rope_abc_start_id, rope_abcd_start_id);
    }
}
