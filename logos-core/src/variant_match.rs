pub trait VariantMatch: std::fmt::Debug + PartialEq {
    fn priority(&self) -> usize;
}

#[derive(Debug, PartialEq)]
pub struct SimpleVariantMatch<'a> {
    name: &'a str,
    priority: usize,
}

impl<'a> VariantMatch for SimpleVariantMatch<'a> {
    fn priority(&self) -> usize {
        self.priority
    }
}

impl<'a> SimpleVariantMatch<'a> {
    pub fn new(name: &'a str, priority: usize) -> Self {
        Self { name, priority }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}
