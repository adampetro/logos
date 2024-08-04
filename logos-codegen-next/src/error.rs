use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use std::borrow::Cow;

pub(crate) struct SpannedError {
    message: Cow<'static, str>,
    span: Span,
}

impl SpannedError {
    pub(crate) fn new(message: impl Into<Cow<'static, str>>, span: Span) -> Self {
        SpannedError {
            message: message.into(),
            span,
        }
    }
}

impl ToTokens for SpannedError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let message = &*self.message;

        tokens.append_all(quote_spanned!(self.span => {
            compile_error!(#message)
        }))
    }
}
