use crate::parser::VariantMatch;
use itertools::Itertools;
use logos_core::{Graph, Node, NodeId};
use std::collections::{hash_map::Entry, HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct BacktrackDistanceAnalysis {
    distances: HashMap<NodeId, Result<usize, HashSet<usize>>>,
}

impl BacktrackDistanceAnalysis {
    pub(crate) fn new(graph: &Graph<VariantMatch>) -> Self {
        let mut distances = HashMap::new();

        graph.iter().for_each(|(node_id, node)| {
            let backtrack_node_id_and_distance = match node {
                Node::Fork(fork) => fork.record_miss_backtrack_idx().map(|backtrack_idx| {
                    (
                        backtrack_idx,
                        Self::distance_to_node_miss(
                            node_id,
                            backtrack_idx,
                            graph,
                            &mut HashSet::new(),
                        ),
                    )
                }),
                Node::Rope(rope) => rope.record_miss_backtrack_idx().map(|backtrack_idx| {
                    (
                        backtrack_idx,
                        Self::distance_to_node_miss(
                            node_id,
                            backtrack_idx,
                            graph,
                            &mut HashSet::new(),
                        ),
                    )
                }),
                Node::VariantMatch(_) => None,
            };

            if let Some((backtrack_node_id, distance)) = backtrack_node_id_and_distance {
                match distances.entry(backtrack_node_id) {
                    Entry::Vacant(entry) => {
                        entry.insert(if distance.len() == 1 {
                            Ok(distance.into_iter().next().unwrap())
                        } else {
                            Err(distance)
                        });
                    }
                    Entry::Occupied(mut entry) => match entry.get_mut() {
                        Ok(existing_distance)
                            if distance.len() == 1
                                && distance.iter().next().unwrap() == existing_distance => {}
                        Err(existing_distances) => {
                            existing_distances.extend(distance);
                        }
                        Ok(existing_distance) => {
                            let mut distance = distance;
                            distance.insert(*existing_distance);
                            *entry.get_mut() = Err(distance);
                        }
                    },
                }
            }
        });

        Self { distances }
    }

    fn distance_to_node_miss(
        node_id: NodeId,
        target_miss: NodeId,
        graph: &Graph<VariantMatch>,
        visited: &mut HashSet<NodeId>,
    ) -> HashSet<usize> {
        if !visited.insert(node_id) {
            return HashSet::new();
        }

        match &graph[node_id] {
            Node::Fork(fork) => match fork.miss() {
                Some(miss) if miss == target_miss => HashSet::from([0]),
                miss => {
                    let miss = miss
                        .map(|miss| {
                            Self::distance_to_node_miss(miss, target_miss, graph, visited)
                                .into_iter()
                        })
                        .into_iter()
                        .flatten();
                    fork.lookup_table()
                        .iter()
                        .flatten()
                        .unique()
                        .flat_map(|&id| {
                            Self::distance_to_node_miss(id, target_miss, graph, visited)
                                .into_iter()
                                .map(|distance| distance + 1)
                        })
                        .chain(miss)
                        .collect()
                }
            },
            Node::Rope(rope) => match rope.miss() {
                Some(miss) if miss == target_miss => HashSet::from([0]),
                miss => {
                    let miss = miss
                        .map(|miss| {
                            Self::distance_to_node_miss(miss, target_miss, graph, visited)
                                .into_iter()
                        })
                        .into_iter()
                        .flatten();
                    Self::distance_to_node_miss(rope.then(), target_miss, graph, visited)
                        .into_iter()
                        .map(|distance| distance + rope.pattern().len())
                        .chain(miss)
                        .collect()
                }
            },
            Node::VariantMatch(_) => HashSet::new(),
        }
    }

    pub(crate) fn static_distance_for_backtrack(&self, node_id: NodeId) -> Option<usize> {
        self.distances
            .get(&node_id)
            .and_then(|distance| match distance {
                Ok(distance) => Some(*distance),
                Err(_) => None,
            })
    }
}
