use indexmap::IndexSet;
use itertools::Itertools;
use logos_core::{Fork, Graph, Node, NodeId, Rope};
use quote::format_ident;
use syn::parse_quote;

mod graph_analysis;
use crate::parser::VariantMatch;
use graph_analysis::BacktrackDistanceAnalysis;

pub(crate) struct Generator<'a> {
    state_idents: Vec<syn::Ident>,
    rope_lookups: IndexSet<[bool; 256]>,
    graph: &'a Graph<'a, VariantMatch<'a>>,
    entrypoint: NodeId,
    uses_fast_loop: bool,
    backtrack_distances: BacktrackDistanceAnalysis,
}

impl<'a> Generator<'a> {
    pub(crate) fn generate(
        enum_ident: &syn::Ident,
        graph: &'a Graph<'a, VariantMatch<'a>>,
        entrypoint: NodeId,
    ) -> syn::Item {
        let mut instance = Self {
            state_idents: graph
                .iter()
                .map(|(id, _)| format_ident!("Node{}", id.to_string()))
                .collect(),
            rope_lookups: IndexSet::new(),
            graph,
            entrypoint,
            uses_fast_loop: false,
            backtrack_distances: BacktrackDistanceAnalysis::new(graph),
        };

        let arms = graph
            .iter()
            .map(|(id, node)| -> syn::Arm {
                let body = instance.match_arm_body(id, node);
                let state = &instance.state_idents[*id];
                parse_quote! {
                    State::#state => #body
                }
            })
            .collect::<Vec<syn::Arm>>();

        let state_enum = instance.generate_state_enum();

        let backtrack_struct = instance.generate_backtrack_struct();

        let backtrack_struct_instantiation: Option<syn::Stmt> =
            backtrack_struct.as_ref().map(|_| {
                parse_quote! {
                    let mut backtrack = Backtrack::default();
                }
            });

        let rope_lookups = instance.generate_rope_lookups();

        let initial_state = &instance.state_idents[*entrypoint];

        let fast_loop_macro = instance.uses_fast_loop.then(Self::fast_loop_macro);

        parse_quote! {
            const _: () = {
                #state_enum
                #backtrack_struct
                #rope_lookups
                #fast_loop_macro

                impl<'source> logos_next::Logos<'source> for #enum_ident {
                    fn lex(lexer: &mut logos_next::Lexer<'source, Self>) -> Option<Result<Self, ()>> {
                        let mut state = State::#initial_state;
                        lexer.trivia();
                        #backtrack_struct_instantiation

                        'outer: loop {
                            match state {
                                #(#arms,)*
                            }
                        }

                        lexer.error();
                        Some(Err(()))
                    }
                }
            };
        }
    }

    fn generate_state_enum(&self) -> syn::ItemEnum {
        let variants = &self.state_idents;

        parse_quote! {
            #[derive(Clone, Copy)]
            pub enum State {
                #(#variants,)*
            }
        }
    }

    fn match_arm_body(&mut self, node_id: NodeId, node: &Node<VariantMatch>) -> syn::Expr {
        match node {
            Node::Fork(fork) => self.fork_match_arm_body(node_id, fork),
            Node::Rope(rope) => self.rope_match_arm_body(node_id, rope),
            Node::VariantMatch(variant_match) => self.variant_match_arm_body(variant_match),
        }
    }

    fn fork_match_arm_body(&mut self, node_id: NodeId, fork: &Fork) -> syn::Expr {
        let lookup_table: [syn::Expr; 256] = fork.lookup_table().map(|maybe_node_id| {
            if let Some(node_id) = maybe_node_id {
                let state_ident = &self.state_idents[*node_id];
                parse_quote!(Some(State::#state_ident))
            } else {
                parse_quote!(None)
            }
        });

        let is_entrypoint = node_id == self.entrypoint;
        let on_miss = self.on_miss(fork.miss());

        let byte_read: syn::Stmt = if is_entrypoint {
            parse_quote! {
                let byte = lexer.read::<u8>()?;
            }
        } else {
            parse_quote! {
                let Some(byte) = lexer.read::<u8>() else {
                    #(#on_miss)*
                };
            }
        };

        let lookup: syn::Expr = parse_quote! {
            if let Some(next_state) = LOOKUP_TABLE[byte as usize] {
                state = next_state;
                lexer.bump_unchecked(1);
            } else {
                #(#on_miss)*
            }
        };

        let record_miss_backtrack_idx: Option<syn::Stmt> = fork
            .record_miss_backtrack_idx()
            .and_then(|node_id| self.record_miss_backtrack_idx(node_id));

        parse_quote! {
            {
                const LOOKUP_TABLE: [Option<State>; 256] = [#(#lookup_table,)*];
                #record_miss_backtrack_idx
                #byte_read
                #lookup
            }
        }
    }

    fn rope_match_arm_body(&mut self, node_id: NodeId, rope: &Rope) -> syn::Expr {
        if rope.pattern().len() == 1 && rope.then() == node_id {
            return self.rope_match_arm_body_fast_loop(rope);
        }

        let pattern_as_slice =
            rope.pattern()
                .iter()
                .try_fold(Vec::new(), |mut bytes, pattern_for_idx| {
                    if pattern_for_idx.len() == 1 {
                        bytes.extend(pattern_for_idx.iter().copied());
                        Ok(bytes)
                    } else {
                        Err(())
                    }
                });

        let record_miss_backtrack_idx: Option<syn::Stmt> = rope
            .record_miss_backtrack_idx()
            .and_then(|node_id| self.record_miss_backtrack_idx(node_id));

        let on_miss = self.on_miss(rope.miss());
        let length = rope.pattern().len();
        let read_bytes: syn::Stmt = parse_quote! {
            let Some(bytes) = lexer.read::<&[u8; #length]>() else {
                #(#on_miss)*
            };
        };
        let then = &self.state_idents[*rope.then()];

        if let Ok(pattern_as_slice) = &pattern_as_slice {
            parse_quote! {
                {
                    #record_miss_backtrack_idx
                    #read_bytes
                    if bytes == &[#(#pattern_as_slice,)*] {
                        state = State::#then;
                        lexer.bump_unchecked(#length);
                    } else {
                        #(#on_miss)*
                    }
                }
            }
        } else {
            let rope_lookup_idxs = rope.pattern().iter().map(|pattern_for_idx| {
                let mut lookup_table = [false; 256];
                pattern_for_idx.iter().for_each(|byte| {
                    lookup_table[*byte as usize] = true;
                });

                let (idx, _is_newly_inserted) = self.rope_lookups.insert_full(lookup_table);
                idx
            });

            let byte_checks = rope_lookup_idxs
                .enumerate()
                .map(|(bytes_idx, rope_lookup_idx)| {
                    let outer_idx = rope_lookup_idx / 8;
                    let inner_idx = rope_lookup_idx % 8;
                    parse_quote! {
                        (ROPE_LOOKUPS[#outer_idx][bytes[#bytes_idx] as usize] & (1 << #inner_idx) != 0)
                    }
                })
                .collect::<Vec<syn::Expr>>();

            parse_quote! {
                {
                    #record_miss_backtrack_idx
                    #read_bytes
                    if #(#byte_checks)&&* {
                        state = State::#then;
                        lexer.bump_unchecked(1);
                    } else {
                        #(#on_miss)*
                    }
                }

            }
        }
    }

    fn variant_match_arm_body(&self, variant_match: &VariantMatch) -> syn::Expr {
        let ident = variant_match.name;
        parse_quote! {
            {
                return Some(Ok(Self::#ident));
            }
        }
    }

    fn rope_match_arm_body_fast_loop(&mut self, rope: &Rope) -> syn::Expr {
        let [pattern] = rope.pattern() else {
            unreachable!("Rope pattern has more than one byte");
        };

        self.uses_fast_loop = true;

        let test_defn: syn::Stmt = if pattern.len() == 1 {
            let byte = pattern.iter().next().copied().unwrap();
            parse_quote! {
                let test = |byte: u8| byte == #byte;
            }
        } else {
            let mut lookup_table = [false; 256];
            pattern.iter().for_each(|byte| {
                lookup_table[*byte as usize] = true;
            });

            let (idx, _is_newly_inserted) = self.rope_lookups.insert_full(lookup_table);

            let outer_idx = idx / 8;
            let inner_idx = idx % 8;

            parse_quote! {
                let test = |byte: u8| ROPE_LOOKUPS[#outer_idx][byte as usize] & (1 << #inner_idx) != 0;
            }
        };

        let record_miss_backtrack_idx: Option<syn::Stmt> = rope
            .record_miss_backtrack_idx()
            .and_then(|node_id| self.record_miss_backtrack_idx(node_id));

        let on_miss = record_miss_backtrack_idx
            .into_iter()
            .chain(self.on_miss(rope.miss()))
            .collect::<Vec<_>>();

        parse_quote! {
            {
                #test_defn
                _fast_loop!(lexer, test, #(#on_miss)*);
            }
        }
    }

    fn generate_backtrack_struct(&self) -> Option<syn::ItemStruct> {
        let backtrack_idxs = self
            .graph
            .iter()
            .filter_map(|(_, node)| match node {
                Node::Fork(fork) => fork.record_miss_backtrack_idx(),
                Node::Rope(rope) => rope.record_miss_backtrack_idx(),
                Node::VariantMatch(_) => None,
            })
            .unique()
            .filter(|&idx| {
                self.backtrack_distances
                    .static_distance_for_backtrack(idx)
                    .is_none()
            })
            .map(Self::backtrack_ident)
            .collect::<Vec<syn::Ident>>();

        (!backtrack_idxs.is_empty()).then(|| {
            parse_quote! {
                #[derive(Default)]
                struct Backtrack {
                    #(#backtrack_idxs: usize,)*
                }
            }
        })
    }

    fn backtrack_ident(node_id: NodeId) -> syn::Ident {
        format_ident!("backtrack_{}", node_id.to_string())
    }

    fn record_miss_backtrack_idx(&self, node_id: NodeId) -> Option<syn::Stmt> {
        match self
            .backtrack_distances
            .static_distance_for_backtrack(node_id)
        {
            Some(_) => None,
            None => {
                let backtrack_ident = Self::backtrack_ident(node_id);
                Some(parse_quote! { backtrack.#backtrack_ident = lexer.current_end(); })
            }
        }
    }

    fn on_miss(&self, miss: Option<NodeId>) -> Vec<syn::Stmt> {
        if let Some(miss) = miss {
            let miss_ident = &self.state_idents[*miss];
            let backtrack: Option<syn::Stmt> =
                match self.backtrack_distances.static_distance_for_backtrack(miss) {
                    Some(0) => None,
                    Some(distance) => Some(parse_quote! {
                        lexer.set_end_unchecked(lexer.current_end() - #distance);
                    }),
                    None => {
                        let backtrack_ident = Self::backtrack_ident(miss);
                        Some(parse_quote! {
                            lexer.set_end_unchecked(backtrack.#backtrack_ident);
                        })
                    }
                };
            parse_quote! {
                state = State::#miss_ident;
                #backtrack
                continue 'outer;
            }
        } else {
            parse_quote!(break 'outer;)
        }
    }

    fn generate_rope_lookups(&self) -> syn::ItemConst {
        let lookups =
            self.rope_lookups
                .iter()
                .chunks(8)
                .into_iter()
                .map(|chunk| {
                    let mut lookup_table = [0u8; 256];
                    chunk.enumerate().for_each(|(i, lookup)| {
                        lookup_table.iter_mut().zip_eq(lookup.iter()).for_each(
                            |(to_update, value)| {
                                *to_update |= (*value as u8) << i;
                            },
                        );
                    });
                    parse_quote! { [#(#lookup_table,)*] }
                })
                .collect::<Vec<syn::ExprArray>>();

        let length = lookups.len();

        parse_quote! {
            const ROPE_LOOKUPS: [[u8; 256]; #length] = [#(#lookups,)*];
        }
    }

    fn fast_loop_macro() -> syn::Item {
        parse_quote! {
            macro_rules! _fast_loop {
                ($lex:ident, $test:ident, $($miss:stmt)*) => {
                    // Do one bounds check for multiple bytes till EOF
                    while let Some(arr) = $lex.read::<&[u8; 16]>() {
                        if $test(arr[0])  { if $test(arr[1])  { if $test(arr[2])  { if $test(arr[3]) {
                        if $test(arr[4])  { if $test(arr[5])  { if $test(arr[6])  { if $test(arr[7]) {
                        if $test(arr[8])  { if $test(arr[9])  { if $test(arr[10]) { if $test(arr[11]) {
                        if $test(arr[12]) { if $test(arr[13]) { if $test(arr[14]) { if $test(arr[15]) {

                        $lex.bump_unchecked(16); continue;     } $lex.bump_unchecked(15); $($miss)* }
                        $lex.bump_unchecked(14); $($miss)* } $lex.bump_unchecked(13); $($miss)* }
                        $lex.bump_unchecked(12); $($miss)* } $lex.bump_unchecked(11); $($miss)* }
                        $lex.bump_unchecked(10); $($miss)* } $lex.bump_unchecked(9); $($miss)*  }
                        $lex.bump_unchecked(8); $($miss)*  } $lex.bump_unchecked(7); $($miss)*  }
                        $lex.bump_unchecked(6); $($miss)*  } $lex.bump_unchecked(5); $($miss)*  }
                        $lex.bump_unchecked(4); $($miss)*  } $lex.bump_unchecked(3); $($miss)*  }
                        $lex.bump_unchecked(2); $($miss)*  } $lex.bump_unchecked(1); $($miss)*  }

                        $($miss)*
                    }

                    while $lex.test($test) {
                        $lex.bump_unchecked(1);
                    }

                    $($miss)*
                }
            }
        }
    }
}
