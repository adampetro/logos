use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote};

#[proc_macro_derive(Logos, attributes(logos, extras, error, end, token, regex))]
pub fn logos(input: TokenStream) -> TokenStream {
    let enum_definition = parse_macro_input!(input as syn::ItemEnum);

    let name = &enum_definition.ident;

    let result: syn::Result<syn::Item> = Ok(parse_quote! {
        impl<'source> logos_next::Logos<'source> for #name {
            fn lex(lexer: &mut logos_next::Lexer<'source, Self>) -> Option<Result<Self, ()>> {
                None
            }
        }
    });

    match result {
        Ok(item) => item.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
