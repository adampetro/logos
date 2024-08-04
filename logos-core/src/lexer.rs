use crate::Variant;
use itertools::Itertools;

pub struct Lexer<T> {
    variants: Vec<Variant<T>>,
}

impl<T> Lexer<T> {
    pub fn new(variants: Vec<Variant<T>>) -> Result<Self, InvalidLexerError>
    where
        T: Eq + std::hash::Hash,
    {
        if variants.is_empty() {
            return Err(InvalidLexerError::NoVariants);
        }
        if !variants.iter().map(Variant::name).all_unique() {
            return Err(InvalidLexerError::NonUniqueVariantNames);
        }
        Ok(Self { variants })
    }

    pub(crate) fn variants(&self) -> &[Variant<T>] {
        &self.variants
    }
}

#[derive(Debug, PartialEq)]
pub enum InvalidLexerError {
    NoVariants,
    NonUniqueVariantNames,
}
