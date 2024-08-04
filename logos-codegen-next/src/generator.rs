use itertools::Itertools;
use logos_core::{Fork, Graph, Node, NodeId, Rope, VariantMatch};
use quote::format_ident;
use syn::parse_quote;

pub(crate) struct Generator<'a> {
    state_idents: Vec<syn::Ident>,
    rope_lookups: Vec<[bool; 256]>,
    graph: &'a Graph<&'a syn::Ident>,
    entrypoint: NodeId,
}

impl<'a> Generator<'a> {
    pub(crate) fn generate(
        enum_ident: &syn::Ident,
        graph: &'a Graph<&'a syn::Ident>,
        entrypoint: NodeId,
    ) -> syn::Item {
        let mut instance = Self {
            state_idents: graph
                .iter()
                .map(|(id, _)| format_ident!("Node{}", id.to_string()))
                .collect(),
            rope_lookups: Vec::new(),
            graph,
            entrypoint,
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

        parse_quote! {
            const _: () = {
                #state_enum
                #backtrack_struct
                #rope_lookups

                impl<'source> logos_next::Logos<'source> for #enum_ident {
                    fn lex(lexer: &mut logos_next::Lexer<'source, Self>) -> Option<Result<Self, ()>> {
                        let mut state = State::#initial_state;
                        lexer.trivia();
                        #backtrack_struct_instantiation

                        loop {
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

    fn match_arm_body(&mut self, node_id: NodeId, node: &Node<&syn::Ident>) -> syn::Expr {
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
                let Some(byte) = lexer.read::<u8>() else {
                    return None;
                };
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

        let record_miss_backtrack_idx: Option<syn::Stmt> =
            fork.record_miss_backtrack_idx().map(|node_id| {
                let backtrack_ident = Self::backtrack_ident(node_id);
                parse_quote! {
                    backtrack.#backtrack_ident = lexer.current_end();
                }
            });

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

        let record_miss_backtrack_idx: Option<syn::Stmt> =
            rope.record_miss_backtrack_idx().map(|node_id| {
                let backtrack_ident = Self::backtrack_ident(node_id);
                parse_quote! {
                    backtrack.#backtrack_ident = lexer.current_end();
                }
            });

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
            let rope_lookup_start = self.rope_lookups.len();
            rope.pattern().iter().for_each(|pattern_for_idx| {
                let mut lookup_table = [false; 256];
                pattern_for_idx.iter().for_each(|byte| {
                    lookup_table[*byte as usize] = true;
                });

                self.rope_lookups.push(lookup_table);
            });

            let byte_checks = (0..length)
                .map(|idx| {
                    let outer_idx = (rope_lookup_start + idx) / 8;
                    let inner_idx = (rope_lookup_start + idx) % 8;
                    parse_quote! {
                        (ROPE_LOOKUPS[#outer_idx][bytes[#idx] as usize] & (1 << #inner_idx) != 0)
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

    fn variant_match_arm_body(&self, variant_match: &VariantMatch<&syn::Ident>) -> syn::Expr {
        let ident = variant_match.variant_name();
        parse_quote! {
            {
                return Some(Ok(Self::#ident));
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

    fn on_miss(&self, miss: Option<NodeId>) -> Vec<syn::Stmt> {
        if let Some(miss) = miss {
            let miss_ident = &self.state_idents[*miss];
            let backtrack_ident = Self::backtrack_ident(miss);
            parse_quote! {
                state = State::#miss_ident;
                lexer.set_end_unchecked(backtrack.#backtrack_ident);
                continue;
            }
        } else {
            parse_quote!(break;)
        }
    }

    fn generate_rope_lookups(&self) -> syn::ItemConst {
        let lookups =
            self.rope_lookups
                .chunks(8)
                .map(|chunk| {
                    let mut lookup_table = [0u8; 256];
                    chunk.iter().enumerate().for_each(|(i, lookup)| {
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
}
