use proc_macro2::TokenStream;
use quote::quote;

use crate::graph::Rope;
use crate::new_generator::Generator;

impl<'a> Generator<'a> {
    pub fn generate_rope(&self, rope: &Rope) -> TokenStream {
        let miss = self.miss(rope.miss.first());
        let read = self.read(rope.pattern.len());
        let then = {
            let advance = self.advance(rope.pattern.len());
            let goto = self.goto(rope.then);
            quote! {
                #advance
                #goto
            }
        };

        let pat = match rope.pattern.to_bytes() {
            Some(bytes) => byte_slice_literal(&bytes),
            None => {
                let ranges = rope.pattern.iter();

                quote!([#(#ranges),*])
            }
        };

        quote! {
            match #read {
                Some(#pat) => {#then}
                _ => #miss,
            }
        }
    }
}

fn byte_slice_literal(bytes: &[u8]) -> TokenStream {
    if bytes.iter().any(|&b| !(0x20..0x7F).contains(&b)) {
        return quote!(&[#(#bytes),*]);
    }

    let slice = std::str::from_utf8(bytes).unwrap();

    syn::parse_str(&format!("b{:?}", slice)).unwrap()
}
