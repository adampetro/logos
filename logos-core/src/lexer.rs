use crate::Variant;
use itertools::Itertools;

pub struct Lexer {
    variants: Vec<Variant>,
}

impl Lexer {
    pub fn new(variants: Vec<Variant>) -> Result<Self, InvalidLexerError> {
        if variants.is_empty() {
            return Err(InvalidLexerError::NoVariants);
        }
        if !variants.iter().map(Variant::name).all_unique() {
            return Err(InvalidLexerError::NonUniqueVariantNames);
        }
        if variants.iter().combinations(2).any(|pair| {
            let a = pair[0];
            let b = pair[1];
            a.specification().can_overlap(b.specification()) && a.priority() == b.priority()
        }) {
            return Err(InvalidLexerError::NonUniqueVariantPriorities);
        }
        Ok(Self { variants })
    }

    pub(crate) fn variants(&self) -> &[Variant] {
        &self.variants
    }
}

#[derive(Debug, PartialEq)]
pub enum InvalidLexerError {
    NoVariants,
    NonUniqueVariantNames,
    NonUniqueVariantPriorities,
}
