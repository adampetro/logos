#[derive(Debug)]
pub(crate) struct VariantMatch<T> {
    pub(crate) variant_name: T,
    pub(crate) priority: usize,
}
