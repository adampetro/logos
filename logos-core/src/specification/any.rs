use crate::Specification;

pub struct Any {
    specifications: Vec<Specification>,
}

impl Any {
    pub(crate) fn default_priority(&self) -> usize {
        self.specifications
            .iter()
            .map(Specification::default_priority)
            .min()
            .unwrap_or(0)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Specification> {
        self.specifications.iter()
    }

    pub(crate) fn new(specifications: Vec<Specification>) -> Self {
        Self { specifications }
    }
}
