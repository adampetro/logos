#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    name: &'a str,
    value: &'a [u8],
}

impl<'a> Token<'a> {
    pub fn new(name: &'a str, value: &'a [u8]) -> Self {
        Self { name, value }
    }
}
