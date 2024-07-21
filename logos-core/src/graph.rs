mod arena;
mod fork;
mod node;
mod rope;
mod variant_match;

use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use crate::{Lexer, Specification, Variant};
use arena::Arena;
pub(crate) use arena::NodeId;
use fork::Fork;
pub(crate) use node::Node;
use rope::Rope;
use variant_match::VariantMatch;

#[derive(Debug)]
struct ReservedId(NodeId);

#[derive(Debug)]
pub(crate) struct Graph {
    nodes: Arena<Option<Node>>,
}

impl Graph {
    pub(crate) fn for_lexer(lexer: &Lexer) -> (Self, NodeId) {
        let mut instance = Self {
            nodes: Arena::default(),
        };
        let mut start_fork = Fork::default();

        // sort variants by decreasing priority
        let mut variants = lexer.variants().iter().collect::<Vec<_>>();
        variants.sort_by_key(|variant| usize::MAX - variant.priority());

        variants.into_iter().for_each(|variant| {
            let fork_for_variant = instance.fork_for_variant(variant);
            start_fork.merge(fork_for_variant, &mut instance);
        });

        let start_node_id = instance.insert(start_fork);

        let start_node_id = instance.shake(start_node_id);

        (instance, start_node_id)
    }

    fn insert(&mut self, node: impl Into<Node>) -> NodeId {
        self.nodes.insert(Some(node.into()))
    }

    fn insert_reserved(&mut self, reserved_id: ReservedId, node: impl Into<Node>) -> NodeId {
        self.nodes[reserved_id.0] = Some(node.into());

        // TODO: handle deferred merges

        reserved_id.0
    }

    fn fork_for_variant(&mut self, variant: &Variant) -> Fork {
        let terminal = self.insert(VariantMatch {
            variant_name: variant.name().to_owned(),
            priority: variant.priority(),
        });

        let node = self.node_for_specification(variant.specification(), terminal, None);

        match node {
            Node::Fork(fork) => fork,
            Node::Rope(rope) => {
                let mut fork = Fork::default().with_miss(rope.miss);
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
    ) -> Node {
        match specification {
            Specification::Byte(value) => Rope {
                pattern: vec![HashSet::from([*value])],
                then,
                miss,
            }
            .into(),
            Specification::Any(any) => Fork::try_from_any(any, then, miss)
                .map(Node::from)
                .unwrap_or_else(|| {
                    let mut fork = Fork::default().with_miss(miss);

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
            Specification::Sequence(sequence) => Rope::try_from_sequence(sequence, then, miss)
                .map(Node::from)
                .unwrap_or_else(|| {
                    let mut reverse_iterator = sequence.iter().rev();
                    let then_node =
                        self.node_for_specification(reverse_iterator.next().unwrap(), then, miss);
                    reverse_iterator.fold(then_node, |then_node, specification| {
                        let then = self.insert(then_node);
                        self.node_for_specification(specification, then, miss)
                    })
                }),
            Specification::Loop(l) => {
                if let Some(max) = l.max() {
                    let terminal = then;
                    let min = l.min();

                    let last_miss = if min == max { miss } else { Some(terminal) };

                    let then_node =
                        self.node_for_specification(l.specification(), terminal, last_miss);

                    let then_node = (min..(max - 1)).fold(then_node, |then_node, _| {
                        let then = self.insert(then_node);
                        self.node_for_specification(l.specification(), then, Some(terminal))
                    });

                    (0..min).fold(then_node, |then_node, _| {
                        let then = self.insert(then_node);
                        self.node_for_specification(l.specification(), then, miss)
                    })
                } else {
                    let reserved_loop_back_to = self.reserve();

                    let loop_start = self.node_for_specification(
                        l.specification(),
                        reserved_loop_back_to.0,
                        Some(then),
                    );

                    let start_id = self.insert_reserved(reserved_loop_back_to, loop_start);

                    let then_node =
                        self.node_for_specification(l.specification(), start_id, Some(then));

                    (0..l.min()).fold(then_node, |then_node, _| {
                        let then = self.insert(then_node);
                        self.node_for_specification(l.specification(), then, miss)
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
            }
            Node::VariantMatch(_) => {}
            Node::Rope(rope) => {
                self.visit_node(rope.then, seen_nodes);
                if let Some(miss) = rope.miss {
                    self.visit_node(miss, seen_nodes);
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
                self[to_id].as_fork_mut().unwrap().miss = Some(from_id);
                to_id
            }
            (Some(Node::VariantMatch(_)), Some(Node::Rope(_))) => {
                self[to_id].as_rope_mut().unwrap().miss = Some(from_id);
                to_id
            }
            _ => todo!(),
        }
    }

    fn reserve(&mut self) -> ReservedId {
        ReservedId(self.nodes.insert(None))
    }
}

impl Index<NodeId> for Graph {
    type Output = Node;

    fn index(&self, index: NodeId) -> &Self::Output {
        self.nodes[index]
            .as_ref()
            .expect("trying to access reserved node")
    }
}

impl IndexMut<NodeId> for Graph {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        self.nodes[index]
            .as_mut()
            .expect("trying to access reserved node")
    }
}
