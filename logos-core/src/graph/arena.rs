use itertools::Itertools;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NodeId(usize);

pub(crate) struct Arena<T>(Vec<T>);

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Index<NodeId> for Arena<T> {
    type Output = T;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<T> IndexMut<NodeId> for Arena<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl<T> Arena<T> {
    pub(crate) fn insert(&mut self, value: T) -> NodeId {
        let id = NodeId(self.0.len());
        self.0.push(value);
        id
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(id, value)| (NodeId(id), value))
    }

    pub(crate) fn iter_ids(&self) -> impl Iterator<Item = NodeId> {
        (0..self.0.len()).map(NodeId)
    }

    pub(crate) fn delete_nodes(
        &mut self,
        node_ids: impl IntoIterator<Item = NodeId>,
        entry_node_id: NodeId,
    ) -> NodeId
    where
        T: HasNodeIds,
    {
        let mut node_ids_to_delete = node_ids.into_iter().unique().collect::<Vec<_>>();
        node_ids_to_delete.sort();

        node_ids_to_delete.iter().rev().for_each(|&node_id| {
            self.0.remove(node_id.0);
        });

        let get_new_node_id = |node_id: NodeId| {
            let Err(index) = node_ids_to_delete.binary_search(&node_id) else {
                panic!("found node id in node_ids_to_delete")
            };
            NodeId(node_id.0 - index)
        };

        self.0.iter_mut().for_each(|value| {
            value.update_node_ids(|node_id| {
                *node_id = get_new_node_id(*node_id);
            });
        });

        get_new_node_id(entry_node_id)
    }
}

impl<T: Debug> Debug for Arena<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nodes = self.iter().collect::<Vec<_>>();
        f.debug_struct("Arena").field("nodes", &nodes).finish()
    }
}

pub(crate) trait HasNodeIds {
    fn update_node_ids(&mut self, f: impl Fn(&mut NodeId));
}

impl<T: HasNodeIds> HasNodeIds for Option<T> {
    fn update_node_ids(&mut self, f: impl Fn(&mut NodeId)) {
        if let Some(value) = self {
            value.update_node_ids(f);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_insert() {
        let mut arena = Arena::default();

        let id1 = arena.insert(1);
        let id2 = arena.insert(2);

        assert_eq!(arena[id1], 1);
        assert_eq!(arena[id2], 2);
    }

    #[test]
    fn arena_iter() {
        let mut arena = Arena::default();

        let id1 = arena.insert(1);
        let id2 = arena.insert(2);

        let mut iter = arena.iter();

        assert_eq!(iter.next(), Some((id1, &1)));
        assert_eq!(iter.next(), Some((id2, &2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arena_delete_nodes_with_dependencies() {
        #[derive(Debug, PartialEq)]
        struct Node {
            reference: Option<NodeId>,
        }

        impl HasNodeIds for Node {
            fn update_node_ids(&mut self, f: impl Fn(&mut NodeId)) {
                if let Some(reference) = &mut self.reference {
                    f(reference);
                }
            }
        }

        let mut arena = Arena::default();

        let id1 = arena.insert(Node { reference: None });
        let id2 = arena.insert(Node { reference: None });
        arena.insert(Node {
            reference: Some(id2),
        });

        let new_id2 = arena.delete_nodes(std::iter::once(id1), id2);

        assert_eq!(new_id2, id1);

        // id2 is now id1, id3 is now id2
        assert_eq!(arena[id1], Node { reference: None });
        assert_eq!(
            arena[id2],
            Node {
                reference: Some(id1)
            }
        );
    }
}
