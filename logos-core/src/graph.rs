mod arena;
mod fork;
mod node;
mod rope;
mod variant_match;

use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use crate::{Lexer, Specification};
use arena::Arena;
pub(crate) use arena::NodeId;
use fork::{Fork, LOOKUP_TABLE_SIZE};
pub(crate) use node::Node;
use rope::Rope;
use variant_match::VariantMatch;

#[derive(Debug)]
struct ReservedId(NodeId);

#[derive(Debug)]
pub(crate) struct Graph<T> {
    nodes: Arena<Option<Node<T>>>,
}

impl<T: Clone> Graph<T> {
    pub(crate) fn for_lexer(lexer: &Lexer<T>) -> (Self, NodeId) {
        let mut instance = Self {
            nodes: Arena::default(),
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

        let start_node_id = instance.shake(start_node_id);

        (instance, start_node_id)
    }

    fn insert(&mut self, node: impl Into<Node<T>>) -> NodeId {
        self.nodes.insert(Some(node.into()))
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
        let from = &self.nodes[from_id];
        let to = &self.nodes[to_id];

        match (from, to) {
            // assume insert in inverse order of priority, so the priority of
            // the to node is always higher
            (_, Some(Node::VariantMatch(_))) => to_id,
            (Some(Node::VariantMatch(_)), Some(Node::Fork(_))) => {
                self.propagate_miss(to_id, from_id, &mut Default::default());
                self[to_id]
                    .as_fork_mut()
                    .unwrap()
                    .record_miss_backtrack_idx
                    .get_or_insert(from_id);
                to_id
            }
            (Some(Node::VariantMatch(_)), Some(Node::Rope(_))) => {
                self.propagate_miss(to_id, from_id, &mut Default::default());
                self[to_id]
                    .as_rope_mut()
                    .unwrap()
                    .record_miss_backtrack_idx
                    .get_or_insert(from_id);
                to_id
            }
            (Some(Node::Rope(from_rope)), Some(Node::Rope(to_rope))) => {
                let from_rope = from_rope.clone();
                let to_rope = to_rope.clone();
                let from_fork = from_rope.fork_off(self);
                let mut to_fork = to_rope.fork_off(self);

                to_fork.merge(from_fork, self);

                self.insert(to_fork)
            }
            (a, b) => {
                todo!()
            }
        }
    }

    fn reserve(&mut self) -> ReservedId {
        ReservedId(self.nodes.insert(None))
    }

    fn propagate_miss(&mut self, node_id: NodeId, miss: NodeId, visited: &mut HashSet<NodeId>) {
        if !visited.insert(node_id) {
            return;
        }

        match &self[node_id] {
            Node::Fork(_) => {
                if self[node_id].as_fork().unwrap().miss.is_none() {
                    self[node_id].as_fork_mut().unwrap().miss = Some(miss);
                    (0..LOOKUP_TABLE_SIZE).for_each(|idx| {
                        if let Some(lookup_node_id) =
                            self[node_id].as_fork().unwrap().lookup_table[idx]
                        {
                            self.propagate_miss(lookup_node_id, miss, visited);
                            if let Some(fork_miss) = self[node_id].as_fork().unwrap().miss {
                                if self.loops_to_target(
                                    lookup_node_id,
                                    node_id,
                                    &mut Default::default(),
                                ) {
                                    self.propagate_miss(fork_miss, miss, visited);
                                }
                            }
                        }
                    });
                }
            }
            Node::VariantMatch(_) => {}
            Node::Rope(_) => {
                if self[node_id].as_rope().unwrap().miss.is_none() {
                    self[node_id].as_rope_mut().unwrap().miss = Some(miss);
                    self.propagate_miss(self[node_id].as_rope().unwrap().then, miss, visited);
                }

                if let Some(rope_miss) = self[node_id].as_rope().unwrap().miss {
                    if self.loops_to_target(
                        node_id,
                        self[node_id].as_rope().unwrap().then,
                        &mut Default::default(),
                    ) {
                        self.propagate_miss(rope_miss, miss, visited);
                    }
                }
            }
        }
    }

    fn loops_to_target(
        &self,
        node_id: NodeId,
        target: NodeId,
        visited: &mut HashSet<NodeId>,
    ) -> bool {
        if node_id == target {
            return true;
        }

        if !visited.insert(node_id) {
            return false;
        }

        match &self[node_id] {
            Node::Fork(fork) => fork.lookup_table.iter().any(|node_id| {
                node_id.map_or(false, |node_id| {
                    self.loops_to_target(node_id, target, visited)
                })
            }),
            Node::VariantMatch(_) => false,
            Node::Rope(rope) => self.loops_to_target(rope.then, target, visited),
        }
    }
}

impl<T> Index<NodeId> for Graph<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeId) -> &Self::Output {
        self.nodes[index]
            .as_ref()
            .expect("trying to access reserved node")
    }
}

impl<T> IndexMut<NodeId> for Graph<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        self.nodes[index]
            .as_mut()
            .expect("trying to access reserved node")
    }
}
