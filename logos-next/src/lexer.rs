use crate::Logos;
use std::marker::PhantomData;

pub struct Lexer<'source, Token: Logos<'source>> {
    source: &'source str,
    token_type: PhantomData<Token>,
}

impl<'source, Token: Logos<'source>> Lexer<'source, Token> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            token_type: PhantomData,
        }
    }
}

impl<'source, Token: Logos<'source>> Iterator for Lexer<'source, Token> {
    type Item = Result<Token, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        Token::lex(self)
    }
}
