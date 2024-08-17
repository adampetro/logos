use crate::VariantMatch;

pub struct Lexer<T: VariantMatch> {
    variant_matches: Vec<T>,
}

impl<T: VariantMatch> Lexer<T> {
    pub fn new(variant_matches: Vec<T>) -> Result<Self, InvalidLexerError> {
        if variant_matches.is_empty() {
            return Err(InvalidLexerError::NoVariants);
        }
        Ok(Self { variant_matches })
    }

    pub fn variant_matches(&self) -> &[T] {
        &self.variant_matches
    }
}

#[derive(Debug, PartialEq)]
pub enum InvalidLexerError {
    NoVariants,
}
