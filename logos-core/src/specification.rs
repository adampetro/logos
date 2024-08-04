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

    pub fn ascii_hex_digit() -> Self {
        Self::Any(Any::new(
            (b'0'..=b'9')
                .chain(b'a'..=b'f')
                .chain(b'A'..=b'F')
                .map(Self::Byte)
                .collect(),
        ))
    }

    pub fn new_str_sequence(s: &str) -> Self {
        if let [byte] = s.as_bytes() {
            Self::Byte(*byte)
        } else {
            Self::Sequence(Sequence::new_str_sequence(s))
        }
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

    pub fn new_not(bytes_disallowed: &[u8]) -> Self {
        let mut bytes_allowed = [true; 256];
        bytes_disallowed.iter().for_each(|&byte| {
            bytes_allowed[byte as usize] = false;
        });
        Self::new_any(
            bytes_allowed
                .iter()
                .enumerate()
                .filter(|(_, &allowed)| allowed)
                .map(|(byte, _)| Specification::Byte(byte as u8))
                .collect(),
        )
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
}
