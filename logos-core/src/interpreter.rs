mod token;

use crate::{
    graph::{Error, Graph, Node, NodeId},
    Lexer, SimpleVariantMatch,
};
use std::collections::HashMap;
pub use token::Token;

#[derive(Debug)]
pub struct Interpreter<'a> {
    graph: Graph<'a, SimpleVariantMatch<'a>>,
    bytes: &'a [u8],
    current_idx: usize,
    backtrack_idxs: HashMap<NodeId, usize>,
}

impl<'a> Interpreter<'a> {
    pub fn new(
        lexer: &'a Lexer<SimpleVariantMatch<'a>>,
        bytes: &'a [u8],
    ) -> Result<Self, Vec<Error<'a, SimpleVariantMatch<'a>>>> {
        // TODO: handle errors
        let graph = Graph::for_lexer(lexer)?;
        Ok(Self {
            graph,
            bytes,
            current_idx: 0,
            backtrack_idxs: HashMap::new(),
        })
    }
}

impl<'a> Iterator for Interpreter<'a> {
    type Item = Result<Token<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.bytes.len() {
            return None;
        }

        let mut idx = self.current_idx;
        let mut current_node_id = self.graph.start_node_id();
        let mut current_node = &self.graph[current_node_id];

        loop {
            dbg!(idx, current_node_id);
            match current_node {
                Node::Fork(fork) => {
                    if let Some(backtrack_idx) = fork.record_miss_backtrack_idx() {
                        self.backtrack_idxs.insert(backtrack_idx, idx);
                    }

                    match self
                        .bytes
                        .get(idx)
                        .and_then(|byte| fork.lookup_table[*byte as usize])
                    {
                        Some(node_id) => {
                            current_node_id = node_id;
                            current_node = &self.graph[node_id];
                            idx += 1;
                        }
                        None => {
                            if let Some(miss) = fork.miss() {
                                dbg!(miss);
                                current_node_id = miss;
                                current_node = &self.graph[miss];
                                idx = self.backtrack_idxs[&miss];
                            } else {
                                break;
                            }
                        }
                    }
                }
                Node::VariantMatch(variant_match) => {
                    let token =
                        Token::new(variant_match.name(), &self.bytes[self.current_idx..idx]);
                    self.current_idx = idx;
                    return Some(Ok(token));
                }
                Node::Rope(rope) => {
                    if let Some(backtrack_idx) = rope.record_miss_backtrack_idx() {
                        self.backtrack_idxs.insert(backtrack_idx, idx);
                    }

                    let matches_pattern =
                        rope.pattern.iter().enumerate().all(|(i, byte_pattern)| {
                            idx + i < self.bytes.len()
                                && byte_pattern.contains(&self.bytes[idx + i])
                        });

                    if matches_pattern {
                        current_node_id = rope.then;
                        current_node = &self.graph[rope.then];
                        idx += rope.pattern.len();
                    } else if let Some(miss) = rope.miss() {
                        dbg!(miss);
                        current_node_id = miss;
                        current_node = &self.graph[miss];
                        idx = self.backtrack_idxs[&miss];
                    } else {
                        break;
                    }
                }
            }
        }

        self.current_idx += 1;

        Some(Err(()))
    }
}
