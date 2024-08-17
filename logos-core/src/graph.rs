mod arena;
mod fork;
mod node;
mod rope;

use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
};

use crate::{Lexer, Specification, VariantMatch};
use arena::Arena;
pub use arena::NodeId;
pub use fork::Fork;
use itertools::Itertools;
pub use node::Node;
pub use rope::Rope;

#[derive(Debug)]
struct ReservedId(NodeId);

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
        let mut builder = GraphBuilder {
            nodes: Arena::default(),
            merges: HashMap::new(),
            clones_with_miss: HashMap::new(),
            errors: Vec::new(),
        };
        let mut start_fork = Fork::new(None, None);

        // sort variants by decreasing priority
        let mut variant_matches: Vec<&'a T> = lexer.variant_matches().iter().collect();
        variant_matches.sort_by_key(|variant_match| usize::MAX - variant_match.priority());

        variant_matches.into_iter().for_each(|variant_match| {
            let fork_for_variant_match = builder.fork_for_variant_match(variant_match);
            start_fork.merge(fork_for_variant_match, &mut builder);
        });

        let start_node_id = builder.insert(start_fork);

        if !builder.errors.is_empty() {
            return Err(builder.errors);
        }

        builder.fork_to_rope();

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
pub struct GraphBuilder<'a, T: VariantMatch> {
    nodes: Arena<Option<Node<'a, T>>>,
    merges: HashMap<[NodeId; 2], NodeId>,
    clones_with_miss: HashMap<(NodeId, NodeId), NodeId>,
    errors: Vec<Error<'a, T>>,
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

        // TODO: handle deferred merges

        reserved_id.0
    }

    fn fork_for_variant_match(&mut self, variant_match: &'a T) -> Fork {
        let terminal = self.insert(variant_match);

        let node = self.node_for_specification(variant_match.specification(), terminal, None, None);

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
    ) -> Node<'a, T> {
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

    fn fork_off(&mut self, node_id: NodeId) -> Fork {
        match &self[node_id] {
            Node::Fork(fork) => fork.clone(),
            Node::Rope(rope) => rope.clone().fork_off(self),
            Node::VariantMatch(_) => Fork::new(Some(node_id), Some(node_id)),
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

        dbg!(&from, &to);

        let merge_id = match (from, to) {
            (Some(Node::VariantMatch(from)), Some(Node::VariantMatch(to))) => {
                if from == to {
                    to_id
                } else if from.priority() == to.priority() {
                    self.errors
                        .push(Error::VariantMatchesOverlapWithSamePriority(from, to));
                    to_id
                } else if from.priority() > to.priority() {
                    from_id
                } else {
                    to_id
                }
            }
            (_, Some(Node::VariantMatch(_))) => self.clone_with_miss(from_id, to_id, true),
            (Some(Node::VariantMatch(_)), _) => self.clone_with_miss(to_id, from_id, true),
            (Some(_), Some(_)) => {
                let from_fork = self.fork_off(from_id);
                let mut to_fork = self.fork_off(to_id);
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
        self.nodes.iter_mut().for_each(|(_, node)| {
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
}

impl<'a, T: VariantMatch> Index<NodeId> for GraphBuilder<'a, T> {
    type Output = Node<'a, T>;

    fn index(&self, index: NodeId) -> &Self::Output {
        self.nodes[index]
            .as_ref()
            .expect("trying to access reserved node")
    }
}

impl<'a, T: VariantMatch> IndexMut<NodeId> for GraphBuilder<'a, T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        self.nodes[index]
            .as_mut()
            .expect("trying to access reserved node")
    }
}
