use proc_macro2::TokenStream;
use quote::quote;

use crate::leaf::{Callback, Leaf};
use crate::new_generator::Generator;
use crate::util::MaybeVoid;

impl<'a> Generator<'a> {
    pub fn generate_leaf(&self, leaf: &Leaf) -> TokenStream {
        let bump: TokenStream = quote! {
            $lex.bump_unchecked($at);
            $at = 0;
        };

        let ident = &leaf.ident;
        let name = self.name;
        let this = self.this;
        let ty = &leaf.field;

        let constructor = match leaf.field {
            MaybeVoid::Some(_) => quote!(#name::#ident),
            MaybeVoid::Void => quote!(|()| #name::#ident),
        };

        match &leaf.callback {
            Some(Callback::Label(callback)) => quote! {
                #bump
                #callback($lex).construct(#constructor, $lex);
                break;
            },
            Some(Callback::Inline(inline)) => {
                let arg = &inline.arg;
                let body = &inline.body;

                quote! {
                    #bump

                    #[inline]
                    fn callback<'s>(#arg: &mut Lexer<'s>) -> impl CallbackResult<'s, #ty, #this> {
                        #body
                    }

                    callback($lex).construct(#constructor, $lex);
                    break;
                }
            }
            Some(Callback::Skip(_)) => {
                let goto = self.goto(self.root);
                quote! {
                    #bump

                    $lex.trivia();
                    #goto
                }
            }
            None if matches!(leaf.field, MaybeVoid::Void) => {
                quote! {
                    #bump
                    $lex.set(Ok(#name::#ident));
                    break;
                }
            }
            None => quote! {
                #bump
                let token = #name::#ident($lex.slice());
                $lex.set(Ok(token));
                break;
            },
        }
    }
}
