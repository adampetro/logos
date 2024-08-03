#[derive(Debug, PartialEq)]
pub struct Token<'a, T> {
    kind: T,
    value: &'a [u8],
}

impl<'a, T> Token<'a, T> {
    pub fn new(kind: T, value: &'a [u8]) -> Self {
        Self { kind, value }
    }
}
