#[derive(Debug)]
pub struct VariantMatch<T> {
    pub(crate) variant_name: T,
    pub(crate) priority: usize,
}

impl<T> VariantMatch<T> {
    pub fn variant_name(&self) -> &T {
        &self.variant_name
    }

    pub fn priority(&self) -> usize {
        self.priority
    }
}
