#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    kind: String,
    value: &'a [u8],
}

impl<'a> Token<'a> {
    pub fn new(kind: String, value: &'a [u8]) -> Self {
        Self { kind, value }
    }
}
