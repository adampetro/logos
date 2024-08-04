use logos_core::Graph;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

mod error;
mod generator;
mod parser;

use generator::Generator;
use parser::Parser;

#[proc_macro_derive(Logos, attributes(logos, extras, error, end, token, regex))]
pub fn logos(input: TokenStream) -> TokenStream {
    let enum_definition = parse_macro_input!(input as syn::ItemEnum);

    let name = &enum_definition.ident;

    let lexer = match Parser::parse(&enum_definition) {
        Ok(lexer) => lexer,
        Err(err) => {
            return quote!(#(#err)*).into_token_stream().into();
        }
    };

    let (graph, entrypoint) = Graph::for_lexer(&lexer);

    Generator::generate(name, &graph, entrypoint)
        .into_token_stream()
        .into()
}
