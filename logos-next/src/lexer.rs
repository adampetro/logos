use crate::{Logos, Source};
use std::marker::PhantomData;

pub struct Lexer<'source, Token: Logos<'source>> {
    source: &'source str,
    token_type: PhantomData<Token>,
    token_start: usize,
    token_end: usize,
}

impl<'source, Token: Logos<'source>> Lexer<'source, Token> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            token_type: PhantomData,
            token_start: 0,
            token_end: 0,
        }
    }

    #[inline]
    pub fn read<Chunk>(&self) -> Option<Chunk>
    where
        Chunk: crate::Chunk<'source>,
    {
        self.source.read(self.token_end)
    }

    #[inline]
    pub fn bump_unchecked(&mut self, size: usize) {
        debug_assert!(
            self.token_end + size <= self.source.len(),
            "Bumping out of bounds!"
        );

        self.token_end += size;
    }

    #[inline]
    pub fn current_end(&self) -> usize {
        self.token_end
    }

    #[inline]
    pub fn set_end_unchecked(&mut self, end: usize) {
        debug_assert!(
            end <= self.source.len() && end >= self.token_start,
            "Setting end out of bounds!"
        );

        self.token_end = end;
    }

    #[inline]
    pub fn trivia(&mut self) {
        self.token_start = self.token_end;
    }

    #[inline]
    pub fn error(&mut self) {
        self.token_end = self.source.find_boundary(self.token_end);
    }
}

impl<'source, Token: Logos<'source>> Iterator for Lexer<'source, Token> {
    type Item = Result<Token, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        Token::lex(self)
    }
}
