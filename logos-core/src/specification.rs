mod any;
mod r#loop;
mod sequence;

pub use any::Any;
pub use r#loop::Loop;
pub use sequence::Sequence;

#[derive(enum_as_inner::EnumAsInner)]
pub enum Specification {
    Sequence(Sequence),
    Any(Any),
    Loop(Loop),
    Byte(u8),
}

impl Specification {
    pub fn default_priority(&self) -> usize {
        match self {
            Self::Sequence(sequence) => sequence.default_priority(),
            Self::Any(any) => any.default_priority(),
            Self::Loop(l) => l.default_priority(),
            Self::Byte(_) => 2,
        }
    }

    pub fn ascii_alphabetic() -> Self {
        Self::Any(Any::new(
            ('a'..='z')
                .chain('A'..='Z')
                .map(|c| Self::Byte(c as u8))
                .collect(),
        ))
    }

    pub fn ascii_digit() -> Self {
        Self::Any(Any::new((b'0'..=b'9').map(Self::Byte).collect()))
    }

    pub fn new_str_sequence(s: &str) -> Self {
        Self::Sequence(Sequence::new_str_sequence(s))
    }

    pub fn new_sequence(specifications: Vec<Self>) -> Self {
        Self::Sequence(Sequence::new(specifications))
    }

    pub fn new_any(specifications: Vec<Self>) -> Self {
        Self::Any(Any::new(specifications))
    }

    pub fn new_loop(min: usize, max: Option<usize>, specification: Self) -> Self {
        Self::Loop(Loop::new(min, max, specification))
    }

    pub fn maybe(specification: Self) -> Self {
        Self::Loop(Loop::new(0, Some(1), specification))
    }

    pub(crate) fn can_overlap(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Byte(a), Self::Byte(b)) => a == b,
            (Self::Sequence(sequence_a), Self::Sequence(sequence_b)) => sequence_a
                .iter()
                .zip(sequence_b.iter())
                .all(|(a, b)| a.can_overlap(b)),
            (Self::Any(a), b) | (b, Self::Any(a)) => a.iter().any(|a| a.can_overlap(b)),
            (Self::Loop(a), Self::Loop(b)) => {
                b.max().map_or(true, |b_max| a.min() <= b_max)
                    && a.max().map_or(true, |a_max| b.min() <= a_max)
                    && a.specification().can_overlap(b.specification())
            }
            (Self::Sequence(sequence), Self::Loop(l))
            | (Self::Loop(l), Self::Sequence(sequence)) => {
                l.min() <= sequence.len()
                    && sequence.iter().all(|s| s.can_overlap(l.specification()))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Specification;

    #[test]
    fn test_default_priority() {
        // [a-zA-Z]+
        let specification = Specification::new_loop(1, None, Specification::ascii_alphabetic());
        assert_eq!(specification.default_priority(), 2);

        // foobar
        let specification = Specification::new_str_sequence("foobar");
        assert_eq!(specification.default_priority(), 12);

        // (foo|hello)(bar)?
        let specification = Specification::new_sequence(vec![
            Specification::new_any(vec![
                Specification::new_str_sequence("foo"),
                Specification::new_str_sequence("hello"),
            ]),
            Specification::new_loop(0, Some(1), Specification::new_str_sequence("bar")),
        ]);
        assert_eq!(specification.default_priority(), 6);
    }

    #[test]
    fn test_can_overlap() {
        let a = Specification::Byte(b'a');
        let b = Specification::Byte(b'b');
        assert!(!a.can_overlap(&b));

        let a = Specification::Byte(b'a');
        let b = Specification::Byte(b'a');
        assert!(a.can_overlap(&b));

        let a = Specification::new_str_sequence("foo");
        let b = Specification::new_str_sequence("bar");
        assert!(!a.can_overlap(&b));

        let a = Specification::new_str_sequence("foo");
        let b = Specification::new_str_sequence("foobar");
        assert!(a.can_overlap(&b));

        let a = Specification::new_str_sequence("foobar");
        let b = Specification::new_str_sequence("foo");
        assert!(a.can_overlap(&b));

        let a = Specification::new_str_sequence("foo");
        let b = Specification::new_str_sequence("foo");
        assert!(a.can_overlap(&b));

        let a = Specification::new_str_sequence("abc");
        let b = Specification::new_loop(0, Some(1), Specification::ascii_alphabetic());
        assert!(a.can_overlap(&b));

        let a = Specification::new_str_sequence("foo");
        let b = Specification::new_loop(0, Some(1), Specification::new_str_sequence("bar"));
        assert!(!a.can_overlap(&b));
    }
}
