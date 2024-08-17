use crate::error::SpannedError;
use logos_core::{Lexer, Specification};
use regex_syntax::{
    hir::{Class, Dot, Hir, HirKind},
    ParserBuilder,
};
use syn::spanned::Spanned;

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(
        enum_type: &syn::ItemEnum,
    ) -> Result<Lexer<VariantMatch>, Vec<SpannedError>> {
        let mut variant_matches = Vec::new();
        let mut errors = Vec::new();

        enum_type.variants.iter().for_each(|variant| {
            match variant.fields {
                syn::Fields::Unit => {}
                _ => errors.push(SpannedError::new(
                    "Only unit variants are supported",
                    variant.fields.span(),
                )),
            }

            let name = &variant.ident;

            variant.attrs.iter().for_each(|attr| {
                let Some(ident) = attr.path().get_ident().map(|ident| ident.to_string()) else {
                    return;
                };

                match ident.as_str() {
                    "token" => {
                        let (specification, priority) = Self::parse_token(attr).unwrap();
                        variant_matches.push(VariantMatch {
                            name,
                            specification,
                            priority,
                        });
                    }
                    "regex" => {
                        let (specification, priority) = Self::parse_regex(attr).unwrap();
                        variant_matches.push(VariantMatch {
                            name,
                            specification,
                            priority,
                        });
                    }
                    _ => {}
                }
            });
        });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Lexer::new(variant_matches).unwrap())
    }

    fn parse_token(attribute: &syn::Attribute) -> Result<(Specification, usize), ()> {
        let lit_str = attribute.parse_args::<syn::LitStr>().map_err(|_| ())?;
        let specification = Specification::new_str_sequence(&lit_str.value());
        let priority = specification.default_priority();
        Ok((specification, priority))
    }

    fn parse_regex(attribute: &syn::Attribute) -> Result<(Specification, usize), ()> {
        let lit_str = attribute.parse_args::<syn::LitStr>().map_err(|_| ())?;
        let regex = ParserBuilder::new()
            .build()
            .parse(&lit_str.value())
            .map_err(|_| ())?;
        let specification = Self::specification_from_hir(&regex);
        let priority = specification.default_priority();
        Ok((specification, priority))
    }

    fn specification_from_hir(hir: &Hir) -> Specification {
        match hir.kind() {
            HirKind::Empty => Specification::new_sequence(vec![]),
            HirKind::Concat(concat) => Specification::new_sequence(
                concat.iter().map(Self::specification_from_hir).collect(),
            ),
            HirKind::Alternation(alternation) => {
                let specifications = alternation
                    .iter()
                    .map(Self::specification_from_hir)
                    .collect();
                Specification::new_any(specifications)
            }
            HirKind::Literal(literal) => {
                let bytes = literal.0.to_vec();
                Specification::new_sequence(bytes.iter().map(|b| Specification::Byte(*b)).collect())
            }
            HirKind::Repetition(repetition) => Specification::new_loop(
                repetition.min.try_into().unwrap(),
                repetition.max.map(|max| max.try_into().unwrap()),
                Self::specification_from_hir(&repetition.sub),
            ),
            HirKind::Capture(capture) => Self::specification_from_hir(&capture.sub),
            HirKind::Class(class) => match class {
                Class::Bytes(bytes) => Specification::new_any(
                    bytes
                        .iter()
                        .flat_map(|bytes_range| {
                            (bytes_range.start()..=bytes_range.end()).map(Specification::Byte)
                        })
                        .collect(),
                ),
                Class::Unicode(unicode) => Specification::new_any(
                    unicode
                        .iter()
                        .flat_map(|unicode_range| {
                            (unicode_range.start()..=unicode_range.end()).map(|c| {
                                let mut s = String::new();
                                s.push(c);
                                Specification::new_str_sequence(&s)
                            })
                        })
                        .collect(),
                ),
            },
            _ => todo!("unsupported regex syntax {:?}", hir.kind()),
        }
    }
}

pub(crate) struct VariantMatch<'a> {
    pub(crate) name: &'a syn::Ident,
    pub(crate) specification: Specification,
    pub(crate) priority: usize,
}

impl std::fmt::Debug for VariantMatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantMatch")
            .field("name", &self.name)
            .field("priority", &self.priority)
            .finish()
    }
}

impl logos_core::VariantMatch for VariantMatch<'_> {
    fn specification(&self) -> &Specification {
        &self.specification
    }

    fn priority(&self) -> usize {
        self.priority
    }

    fn is_same_variant(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
