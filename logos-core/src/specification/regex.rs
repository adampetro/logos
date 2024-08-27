use beef::lean::Cow;
use lazy_static::lazy_static;
use regex_syntax::{
    hir::{Dot, Hir, HirKind},
    ParserBuilder,
};

pub use regex_syntax::hir::Class;

use crate::Specification;

lazy_static! {
    static ref DOT_UTF8: Hir = Hir::dot(Dot::AnyChar);
    static ref DOT_BYTES: Hir = Hir::dot(Dot::AnyByte);
}

impl Specification {
    pub fn utf8(source: &str) -> Result<Self, Cow<'static, str>> {
        Ok(Self::try_from(
            ParserBuilder::new()
                .build()
                .parse(source)
                .map_err(|err| err.to_string())?,
        )?)
    }

    pub fn utf8_ignore_case(source: &str) -> Result<Self, Cow<'static, str>> {
        Ok(Self::try_from(
            ParserBuilder::new()
                .case_insensitive(true)
                .build()
                .parse(source)
                .map_err(|err| err.to_string())?,
        )?)
    }

    pub fn binary(source: &str) -> Result<Self, Cow<'static, str>> {
        Ok(Self::try_from(
            ParserBuilder::new()
                .utf8(false)
                .unicode(false)
                .build()
                .parse(source)
                .map_err(|err| err.to_string())?,
        )?)
    }

    pub fn binary_ignore_case(source: &str) -> Result<Self, Cow<'static, str>> {
        Ok(Self::try_from(
            ParserBuilder::new()
                .utf8(false)
                .unicode(false)
                .case_insensitive(true)
                .build()
                .parse(source)
                .map_err(|err| err.to_string())?,
        )?)
    }
}

impl TryFrom<Hir> for Specification {
    type Error = &'static str;

    fn try_from(hir: Hir) -> Result<Self, Self::Error> {
        match hir.into_kind() {
            HirKind::Empty => Ok(Self::new_sequence(vec![])),
            HirKind::Concat(concat) => concat
                .into_iter()
                .map(Self::try_from)
                .collect::<Result<Vec<Self>, Self::Error>>()
                .map(Self::new_sequence),
            HirKind::Alternation(alternation) => alternation
                .into_iter()
                .map(Self::try_from)
                .collect::<Result<Vec<Self>, Self::Error>>()
                .map(Self::new_any),
            HirKind::Literal(literal) => {
                let bytes = literal.0.to_vec();
                Ok(Self::new_sequence(
                    bytes.iter().map(|b| Self::Byte(*b)).collect(),
                ))
            }
            HirKind::Class(class) => match class {
                Class::Bytes(bytes) => Ok(Self::new_any(
                    bytes
                        .iter()
                        .flat_map(|bytes_range| {
                            (bytes_range.start()..=bytes_range.end()).map(Self::Byte)
                        })
                        .collect(),
                )),
                Class::Unicode(unicode) => Ok(Self::new_any(
                    unicode
                        .iter()
                        .flat_map(|unicode_range| {
                            (unicode_range.start()..=unicode_range.end()).map(|c| {
                                let mut s = String::new();
                                s.push(c);
                                Self::new_str_sequence(&s)
                            })
                        })
                        .collect(),
                )),
            },
            HirKind::Repetition(repetition) => {
                if !repetition.greedy {
                    return Err("#[regex]: non-greedy parsing is currently unsupported.");
                }

                let is_dot = if repetition.sub.properties().is_utf8() {
                    *repetition.sub == *DOT_UTF8
                } else {
                    *repetition.sub == *DOT_BYTES
                };
                let specification = Self::try_from(*repetition.sub)?;

                match (repetition.min, repetition.max) {
                    (0..=1, None) if is_dot => {
                        Err(
                            "#[regex]: \".+\" and \".*\" patterns will greedily consume \
                            the entire source till the end as Logos does not allow \
                            backtracking. If you are looking to match everything until \
                            a specific character, you should use a negative character \
                            class. E.g., use regex r\"'[^']*'\" to match anything in \
                            between two quotes. Read more about that here: \
                            https://github.com/maciejhirsz/logos/issues/302#issuecomment-1521342541."
                        )
                    }
                    (min, max) => Ok(Self::new_loop(
                        min.try_into().unwrap(),
                        max.map(|max| max.try_into().unwrap()),
                        specification,
                    )),
                }
            }
            HirKind::Capture(capture) => Self::try_from(*capture.sub),
            HirKind::Look(_) => Err("#[regex]: look-around assertions are currently unsupported."),
        }
    }
}
