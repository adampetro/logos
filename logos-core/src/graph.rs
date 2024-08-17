mod arena;
mod fork;
mod node;
mod rope;
mod variant_match;

use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
};

use crate::{Lexer, Specification};
use arena::Arena;
pub use arena::NodeId;
pub use fork::Fork;
use itertools::Itertools;
pub use node::Node;
pub use rope::Rope;
pub use variant_match::VariantMatch;

#[derive(Debug)]
struct ReservedId(NodeId);

#[derive(Debug)]
pub struct Graph<T: Clone + PartialEq> {
    nodes: Arena<Option<Node<T>>>,
    merges: HashMap<[NodeId; 2], NodeId>,
    clones_with_miss: HashMap<(NodeId, NodeId), NodeId>,
}

impl<T: Clone + PartialEq> Graph<T> {
    pub fn for_lexer(lexer: &Lexer<T>) -> (Self, NodeId) {
        let mut instance = Self {
            nodes: Arena::default(),
            merges: Default::default(),
            clones_with_miss: Default::default(),
        };
        let mut start_fork = Fork::new(None, None);

        // sort variants by decreasing priority
        let mut variant_patterns: Vec<(&T, &Specification, usize)> = lexer
            .variants()
            .iter()
            .flat_map(|variant| {
                variant
                    .specifications()
                    .iter()
                    .map(move |(specification, priority)| {
                        (variant.name(), specification, *priority)
                    })
            })
            .collect();
        variant_patterns.sort_by_key(|(_, _, priority)| usize::MAX - priority);

        variant_patterns
            .into_iter()
            .for_each(|(name, specification, priority)| {
                let fork_for_variant_pattern =
                    instance.fork_for_variant_pattern(name, specification, priority);
                start_fork.merge(fork_for_variant_pattern, &mut instance);
            });

        let start_node_id = instance.insert(start_fork);

        instance.fork_to_rope();

        let start_node_id = instance.shake(start_node_id);

        (instance, start_node_id)
    }

    fn insert(&mut self, node: impl Into<Node<T>>) -> NodeId {
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

    fn insert_reserved(&mut self, reserved_id: ReservedId, node: impl Into<Node<T>>) -> NodeId {
        self.nodes[reserved_id.0] = Some(node.into());

        // TODO: handle deferred merges

        reserved_id.0
    }

    fn fork_for_variant_pattern(
        &mut self,
        name: &T,
        specification: &Specification,
        priority: usize,
    ) -> Fork {
        let terminal = self.insert(VariantMatch {
            variant_name: name.clone(),
            priority,
        });

        let node = self.node_for_specification(specification, terminal, None, None);

        match node {
            Node::Fork(fork) => fork,
            Node::Rope(rope) => {
                let mut fork = Fork::new(None, None);
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
        miss: Option<NodeId>,
        record_miss_backtrack_idx: Option<NodeId>,
    ) -> Node<T> {
        match specification {
            Specification::Byte(value) => Rope {
                pattern: vec![HashSet::from([*value])],
                then,
                miss,
                record_miss_backtrack_idx,
            }
            .into(),
            Specification::Any(any) => {
                Fork::try_from_any(any, then, miss, record_miss_backtrack_idx)
                    .map(Node::from)
                    .unwrap_or_else(|| {
                        let mut fork = Fork::new(miss, record_miss_backtrack_idx);

                        any.iter().for_each(|specification| {
                            let node = self.node_for_specification(specification, then, miss, None);
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
                    })
            }
            Specification::Sequence(sequence) => {
                Rope::try_from_sequence(sequence, then, miss, record_miss_backtrack_idx)
                    .map(Node::from)
                    .unwrap_or_else(|| {
                        let sequence_length = sequence.len();
                        let mut reverse_iterator =
                            sequence
                                .iter()
                                .rev()
                                .enumerate()
                                .map(|(idx, specification)| {
                                    ((idx + 1 == sequence_length), specification)
                                });
                        let (is_last, specification) = reverse_iterator.next().unwrap();
                        let then_node = self.node_for_specification(
                            specification,
                            then,
                            miss,
                            is_last.then_some(record_miss_backtrack_idx).flatten(),
                        );
                        reverse_iterator.fold(then_node, |then_node, (is_last, specification)| {
                            let then = self.insert(then_node);
                            self.node_for_specification(
                                specification,
                                then,
                                miss,
                                is_last.then_some(record_miss_backtrack_idx).flatten(),
                            )
                        })
                    })
            }
            Specification::Loop(l) => {
                if let Some(max) = l.max() {
                    let terminal = then;
                    let min = l.min();

                    let last_miss = if min == max { miss } else { Some(terminal) };

                    let then_node = self.node_for_specification(
                        l.specification(),
                        terminal,
                        last_miss,
                        (min != max).then_some(terminal),
                    );

                    let then_node = (min..(max - 1)).fold(then_node, |then_node, _| {
                        let then = self.insert(then_node);
                        self.node_for_specification(
                            l.specification(),
                            then,
                            Some(terminal),
                            Some(terminal),
                        )
                    });

                    (0..min).fold(then_node, |then_node, i| {
                        let then = self.insert(then_node);
                        let is_last = i + 1 == min;
                        self.node_for_specification(
                            l.specification(),
                            then,
                            miss,
                            is_last.then_some(record_miss_backtrack_idx).flatten(),
                        )
                    })
                } else {
                    let reserved_loop_back_to = self.reserve();

                    let loop_end = self.node_for_specification(
                        l.specification(),
                        reserved_loop_back_to.0,
                        Some(then),
                        Some(then),
                    );

                    let start_id = self.insert_reserved(reserved_loop_back_to, loop_end);

                    let then_node = self.node_for_specification(
                        l.specification(),
                        start_id,
                        Some(then),
                        Some(then),
                    );

                    (0..l.min()).fold(then_node, |then_node, i| {
                        let then = self.insert(then_node);
                        let is_last = i + 1 == l.min();
                        self.node_for_specification(
                            l.specification(),
                            then,
                            miss,
                            is_last.then_some(record_miss_backtrack_idx).flatten(),
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
        match &self[node_id] {
            Node::Fork(fork) => {
                fork.lookup_table.iter().for_each(|node_id| {
                    if let Some(node_id) = node_id {
                        self.visit_node(*node_id, seen_nodes);
                    }
                });
                if let Some(miss) = fork.miss {
                    self.visit_node(miss, seen_nodes);
                }
                if let Some(record_miss_backtrack_idx) = fork.record_miss_backtrack_idx {
                    self.visit_node(record_miss_backtrack_idx, seen_nodes);
                }
            }
            Node::VariantMatch(_) => {}
            Node::Rope(rope) => {
                self.visit_node(rope.then, seen_nodes);
                if let Some(miss) = rope.miss {
                    self.visit_node(miss, seen_nodes);
                }
                if let Some(record_miss_backtrack_idx) = rope.record_miss_backtrack_idx {
                    self.visit_node(record_miss_backtrack_idx, seen_nodes);
                }
            }
        }
    }

    pub(crate) fn merge(&mut self, from_id: NodeId, to_id: NodeId) -> NodeId {
        let mut key = [from_id, to_id];
        key.sort();
        if let Some(node_id) = self.merges.get(&key) {
            return *node_id;
        }

        let from = &self.nodes[from_id];
        let to = &self.nodes[to_id];

        let merge_id = match (from, to) {
            // assume insert in inverse order of priority, so the priority of
            // the to node is always higher
            (_, Some(Node::VariantMatch(_))) => to_id,
            (Some(Node::VariantMatch(_)), Some(Node::Fork(_) | Node::Rope(_))) => {
                self.clone_with_miss(to_id, from_id, true)
            }
            (Some(Node::Rope(from_rope)), Some(Node::Rope(to_rope))) => {
                let from_rope = from_rope.clone();
                let to_rope = to_rope.clone();
                let from_fork = from_rope.fork_off(self);
                let mut to_fork = to_rope.fork_off(self);

                to_fork.merge(from_fork, self);

                self.insert(to_fork)
            }
            (Some(Node::Fork(from_fork)), Some(Node::Fork(to_fork))) => {
                let from_fork = from_fork.clone();
                let mut to_fork = to_fork.clone();

                to_fork.merge(from_fork, self);

                self.insert(to_fork)
            }
            (Some(Node::Fork(from_fork)), Some(Node::Rope(to_rope))) => {
                let from_fork = from_fork.clone();
                let to_rope = to_rope.clone();
                let mut to_fork = to_rope.fork_off(self);

                to_fork.merge(from_fork, self);

                self.insert(to_fork)
            }
            (Some(Node::Rope(from_rope)), Some(Node::Fork(to_fork))) => {
                let from_rope = from_rope.clone();
                let mut to_fork = to_fork.clone();
                let from_fork = from_rope.fork_off(self);

                to_fork.merge(from_fork, self);

                self.insert(to_fork)
            }
            (None, Some(_)) | (Some(_), None) | (None, None) => {
                todo!("Deferred merge not yet implemented")
            }
        };

        self.merges.insert(key, merge_id);
        let mut key = [to_id, merge_id];
        key.sort();
        self.merges.insert(key, merge_id);
        let mut key = [from_id, merge_id];
        key.sort();
        self.merges.insert([from_id, merge_id], merge_id);

        merge_id
    }

    fn clone_with_miss(
        &mut self,
        node_id: NodeId,
        miss: NodeId,
        record_miss_backtrack_idx: bool,
    ) -> NodeId {
        if matches!(self.nodes[node_id], Some(Node::VariantMatch(_))) {
            return node_id;
        }

        if matches!(&self.nodes[node_id], Some(Node::Fork(fork)) if fork.miss == Some(miss) && (!record_miss_backtrack_idx || fork.record_miss_backtrack_idx == Some(miss)))
        {
            return node_id;
        }

        if matches!(&self.nodes[node_id], Some(Node::Rope(rope)) if rope.miss == Some(miss) && (!record_miss_backtrack_idx || rope.record_miss_backtrack_idx == Some(miss)))
        {
            return node_id;
        }

        if let Some(node_id) = self.clones_with_miss.get(&(node_id, miss)) {
            return *node_id;
        }

        let deferred = self.reserve();

        self.clones_with_miss.insert((node_id, miss), deferred.0);

        match &self.nodes[node_id] {
            None => panic!("trying to clone reserved node"),
            Some(Node::VariantMatch(_)) => unreachable!("already handled"),
            Some(Node::Fork(fork)) => {
                let mut fork = fork.clone();
                if let Some(fork_miss) = fork.miss {
                    fork.miss = Some(self.clone_with_miss(fork_miss, miss, false));
                } else {
                    fork.miss = Some(miss);
                }
                if record_miss_backtrack_idx {
                    fork.record_miss_backtrack_idx = Some(miss);
                } else if let Some(record_miss_backtrack_idx) = &mut fork.record_miss_backtrack_idx
                {
                    *record_miss_backtrack_idx = self
                        .clones_with_miss
                        .get(&(*record_miss_backtrack_idx, miss))
                        .copied()
                        .unwrap_or(*record_miss_backtrack_idx);
                }
                fork.lookup_table.iter_mut().for_each(|lookup_node_id| {
                    if let Some(lookup_node_id) = lookup_node_id {
                        *lookup_node_id = self.clone_with_miss(*lookup_node_id, miss, false);
                    }
                });
                self.insert_reserved(deferred, fork)
            }
            Some(Node::Rope(rope)) => {
                let mut rope = rope.clone();
                if let Some(rope_miss) = rope.miss {
                    rope.miss = Some(self.clone_with_miss(rope_miss, miss, false));
                } else {
                    rope.miss = Some(miss);
                }
                if record_miss_backtrack_idx {
                    rope.record_miss_backtrack_idx = Some(miss);
                } else if let Some(record_miss_backtrack_idx) = &mut rope.record_miss_backtrack_idx
                {
                    *record_miss_backtrack_idx = self
                        .clones_with_miss
                        .get(&(*record_miss_backtrack_idx, miss))
                        .copied()
                        .unwrap_or(*record_miss_backtrack_idx);
                }
                rope.then = self.clone_with_miss(rope.then, miss, false);
                self.insert_reserved(deferred, rope)
            }
        }
    }

    fn reserve(&mut self) -> ReservedId {
        ReservedId(self.nodes.insert(None))
    }

    fn fork_to_rope(&mut self) {
        self.nodes.iter_mut().for_each(|(node_id, node)| {
            if let Some(Node::Fork(fork)) = node {
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
                    *node = Some(Node::Rope(Rope {
                        pattern: vec![pattern],
                        then,
                        miss: fork.miss,
                        record_miss_backtrack_idx: fork.record_miss_backtrack_idx,
                    }));
                }
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &Node<T>)> {
        self.nodes
            .iter()
            .map(|(id, node)| (id, node.as_ref().unwrap()))
    }
}

impl<T: Clone + PartialEq> Index<NodeId> for Graph<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeId) -> &Self::Output {
        self.nodes[index]
            .as_ref()
            .expect("trying to access reserved node")
    }
}

impl<T: Clone + PartialEq> IndexMut<NodeId> for Graph<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        self.nodes[index]
            .as_mut()
            .expect("trying to access reserved node")
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use crate::{Lexer, Specification, Variant};

    #[test]
    fn test_graph() {
        let lexer = Lexer::new(vec![
            Variant::new("foo", Specification::new_str_sequence("foo"), None),
            // Variant::new("bar", Specification::new_str_sequence("bar"), None),
            // Variant::new(
            //     "number",
            //     Specification::new_loop(1, None, Specification::ascii_digit()),
            //     None,
            // ),
            // Variant::new(
            //     "abcdefghi",
            //     Specification::new_sequence(vec![
            //         Specification::new_any(vec![
            //             Specification::Byte(b'a'),
            //             Specification::Byte(b'b'),
            //             Specification::Byte(b'c'),
            //         ]),
            //         Specification::new_any(vec![
            //             Specification::Byte(b'd'),
            //             Specification::Byte(b'e'),
            //             Specification::Byte(b'f'),
            //         ]),
            //         Specification::new_any(vec![
            //             Specification::Byte(b'g'),
            //             Specification::Byte(b'h'),
            //             Specification::Byte(b'i'),
            //         ]),
            //     ]),
            //     None,
            // ),
            Variant::new(
                "text",
                Specification::new_loop(1, None, Specification::ascii_alphabetic()),
                None,
            ),
        ])
        .unwrap();
        let (graph, _) = Graph::for_lexer(&lexer);
        dbg!(&graph);
        assert_eq!(graph.iter().count(), 0);
    }
}
