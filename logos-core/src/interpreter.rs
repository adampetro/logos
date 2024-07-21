mod token;

use crate::{
    graph::{Graph, Node, NodeId},
    Lexer,
};
pub use token::Token;

#[derive(Debug)]
pub struct Interpreter<'a> {
    graph: Graph,
    start_node_id: NodeId,
    bytes: &'a [u8],
    current_idx: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(lexer: Lexer, bytes: &'a [u8]) -> Self {
        let (graph, start_node_id) = Graph::for_lexer(&lexer);
        Self {
            graph,
            start_node_id,
            bytes,
            current_idx: 0,
        }
    }
}

impl<'a> Iterator for Interpreter<'a> {
    type Item = Result<Token<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.bytes.len() {
            return None;
        }

        let mut idx = self.current_idx;
        let mut current_node_id = self.start_node_id;
        let mut current_node = &self.graph[self.start_node_id];

        loop {
            dbg!(idx, current_node_id);
            match current_node {
                Node::Fork(fork) => {
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
                            if let Some(miss) = fork.miss {
                                dbg!("miss, going to {}", miss);
                                current_node_id = miss;
                                current_node = &self.graph[miss];
                            } else {
                                break;
                            }
                        }
                    }
                }
                Node::VariantMatch(variant_match) => {
                    let token = Token::new(
                        variant_match.variant_name.to_owned(),
                        &self.bytes[self.current_idx..idx],
                    );
                    self.current_idx = idx;
                    return Some(Ok(token));
                }
                Node::Rope(rope) => {
                    let matches_pattern =
                        rope.pattern.iter().enumerate().all(|(i, byte_pattern)| {
                            idx + i < self.bytes.len()
                                && byte_pattern.contains(&self.bytes[idx + i])
                        });

                    if matches_pattern {
                        current_node_id = rope.then;
                        current_node = &self.graph[rope.then];
                        idx += rope.pattern.len();
                    } else if let Some(miss) = rope.miss {
                        dbg!("miss, going to {}", miss);
                        current_node_id = miss;
                        current_node = &self.graph[miss];
                    } else {
                        return Some(Err(()));
                    }
                }
            }
        }

        println!("Encountered error, idx: {}", idx);

        Some(Err(()))
    }
}
