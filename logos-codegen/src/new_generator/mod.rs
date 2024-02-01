use crate::graph::{Graph, Meta, Node, NodeId, Range};
use crate::leaf::Leaf;
use crate::util::ToIdent;
use fnv::FnvHashMap as Map;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

mod fork;
mod leaf;
mod rope;
mod tables;

use tables::TableStack;

pub struct Generator<'a> {
    /// Name of the type we are implementing the `Logos` trait for
    name: &'a Ident,
    /// Name of the type with any generics it might need
    this: &'a TokenStream,
    /// Id to the root node
    root: NodeId,
    /// Reference to the graph with all of the nodes
    graph: &'a Graph<Leaf<'a>>,
    /// Meta data collected for the nodes
    meta: Meta,
    /// Identifiers for states corresponding to nodes
    state_idents: Map<NodeId, Ident>,
    // /// Local function calls.
    // gotos: Map<NodeId, TokenStream>,
    /// Identifiers and implementations for helper functions matching a byte to a given
    /// set of ranges
    tests: Map<Vec<Range>, (Ident, TokenStream)>,
    /// Related to above, table stack manages tables that need to be
    tables: TableStack,
}

impl<'a> Generator<'a> {
    pub fn new(
        name: &'a Ident,
        this: &'a TokenStream,
        root: NodeId,
        graph: &'a Graph<Leaf>,
    ) -> Self {
        let meta = Meta::analyze(root, graph);

        Generator {
            name,
            this,
            root,
            graph,
            meta,
            state_idents: Self::state_idents(graph),
            // gotos: Map::default(),
            tests: Map::default(),
            tables: TableStack::new(),
        }
    }

    pub fn generate(mut self) -> TokenStream {
        let goto_macro = self.goto_macro().clone();
        let states_enum = self.states_enum();
        let states = self.state_idents.values();
        let initial_state = &self.state_idents[&self.root];
        let fast_loop_macro = Self::fast_loop_macro();
        let tests = self.tests.values().map(|(_, test)| test);
        let tables = self.tables;

        let token_stream = quote! {
            #states_enum
            #goto_macro
            #fast_loop_macro
            #(#tests)*
            #tables
            let mut state = State::#initial_state;
            let mut at: usize = 0;

            'state_machine: loop {
                match state {
                    #(State::#states => { goto!(#states, lex, at, state, 'state_machine) })*
                }
            }
        };
        // std::fs::write("generated.rs", token_stream.to_string()).unwrap();
        token_stream
    }

    fn state_idents(graph: &Graph<Leaf>) -> Map<NodeId, Ident> {
        graph
            .node_ids()
            .map(|id| (id, format!("GoTo{id}").to_ident()))
            .collect()
    }

    fn states_enum(&self) -> TokenStream {
        let states = self.state_idents.values();

        quote! {
            enum State {
                #(#states,)*
            }
        }
    }

    fn goto_macro(&mut self) -> TokenStream {
        let states = self.state_idents.values().cloned().collect::<Vec<_>>();
        let ids = self.state_idents.keys().copied().collect::<Vec<NodeId>>();
        let impls = ids.iter().map(|id| self.impl_for_state(*id));

        quote! {
            macro_rules! goto {
                #((#states, $lex:ident, $at:ident, $state:ident, $state_machine:lifetime) => {{#impls}};)*
            }
        }
    }

    // fn gotos(
    //     graph: &Graph<Leaf>,
    //     meta: &Meta,
    //     state_idents: &Map<NodeId, Ident>,
    // ) -> Map<NodeId, TokenStream> {
    //     graph
    //         .node_ids()
    //         .map(|id| {
    //             let meta = &meta[id];
    //             let enters_loop = !meta.loop_entry_from.is_empty();

    //             let bump = if enters_loop { graph[id].miss() } else { None };

    //             let bump = match (bump, enters_loop, meta.min_read) {
    //                 (Some(t), _, _) => Some(t),
    //                 (None, true, _) => ctx.bump(),
    //                 (None, false, 0) => ctx.bump(),
    //                 (None, false, _) => None,
    //             };

    //             if meta.min_read == 0 || ctx.remainder() < meta.min_read {
    //                 ctx.wipe();
    //             }

    //             let ident = &state_idents[&id];
    //             let mut call_site = quote!(#ident(lex));

    //             let mut call_site = quote! {{ $state = State::#state; continue $state_machine;  }};

    //             if let Some(bump) = bump {
    //                 call_site = quote!({
    //                     #bump
    //                     #call_site
    //                 });
    //             }
    //             (id, call_site)
    //         })
    //         .collect()
    // }

    fn impl_for_state(&mut self, id: NodeId) -> TokenStream {
        let node = &self.graph[id];

        match node {
            Node::Fork(fork) => self.generate_fork(id, fork),
            Node::Rope(rope) => self.generate_rope(rope),
            Node::Leaf(leaf) => self.generate_leaf(leaf),
        }
    }

    fn goto(&self, id: NodeId) -> TokenStream {
        let state = &self.state_idents[&id];

        quote! {{ $state = State::#state; continue $state_machine;  }}
    }

    fn miss(&self, miss: Option<NodeId>) -> TokenStream {
        match miss {
            Some(id) => self.goto(id),
            None => {
                quote! {{
                    $lex.bump_unchecked(1);
                    $lex.error();
                    break;
                }}
            }
        }
    }

    fn read(&self, len: usize) -> TokenStream {
        match len {
            0 => quote!($lex.read_at::<u8>($at)),
            l => quote!($lex.read_at::<&[u8; #l]>($at)),
        }
    }

    fn advance(&self, len: usize) -> TokenStream {
        quote!($at += #len;)
    }

    // fn bump(&self) -> TokenStream {
    //     quote! {{
    //         $lex.bump_unchecked($at);
    //         $at = 0;
    //     }}
    // }

    /// Returns an identifier to a function that matches a byte to any
    /// of the provided ranges. This will generate either a simple
    /// match expression, or use a lookup table internally.
    fn generate_test(&mut self, ranges: Vec<Range>) -> &Ident {
        if !self.tests.contains_key(&ranges) {
            let idx = self.tests.len();
            let ident = format!("pattern{}", idx).to_ident();

            let lo = ranges.first().unwrap().start;
            let hi = ranges.last().unwrap().end;

            let body = match ranges.len() {
                0..=2 => {
                    quote! {
                        match byte {
                            #(#ranges)|* => true,
                            _ => false,
                        }
                    }
                }
                _ if hi - lo < 64 => {
                    let mut offset = hi.saturating_sub(63);

                    while offset.count_ones() > 1 && lo - offset > 0 {
                        offset += 1;
                    }

                    let mut table = 0u64;

                    for byte in ranges.iter().flat_map(|range| *range) {
                        if byte - offset >= 64 {
                            panic!("{:#?} {} {} {}", ranges, hi, lo, offset);
                        }
                        table |= 1 << (byte - offset);
                    }

                    let search = match offset {
                        0 => quote!(byte),
                        _ => quote!(byte.wrapping_sub(#offset)),
                    };

                    quote! {
                        const LUT: u64 = #table;

                        match 1u64.checked_shl(#search as u32) {
                            Some(shift) => LUT & shift != 0,
                            None => false,
                        }
                    }
                }
                _ => {
                    let mut view = self.tables.view();

                    for byte in ranges.iter().flat_map(|range| *range) {
                        view.flag(byte);
                    }

                    let mask = view.mask();
                    let lut = view.ident();

                    quote! {
                        #lut[byte as usize] & #mask > 0
                    }
                }
            };
            let implementation = quote! {
                #[inline]
                fn #ident(byte: u8) -> bool {
                    #body
                }
            };
            self.tests.insert(ranges.clone(), (ident, implementation));
        }
        &self.tests[&ranges].0
    }
}
