#[derive(Debug, PartialEq)]
pub struct VariantMatch<T: PartialEq> {
    pub(crate) variant_name: T,
    pub(crate) priority: usize,
}

impl<T: PartialEq> VariantMatch<T> {
    pub fn variant_name(&self) -> &T {
        &self.variant_name
    }

    pub fn priority(&self) -> usize {
        self.priority
    }
}
