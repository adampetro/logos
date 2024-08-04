use crate::Specification;

pub struct Variant<T> {
    name: T,
    specifications: Vec<(Specification, usize)>,
}

impl<T> Variant<T> {
    pub fn new(name: T, specification: Specification, priority: Option<usize>) -> Self {
        let priority = priority.unwrap_or_else(|| specification.default_priority());
        Self {
            name,
            specifications: vec![(specification, priority)],
        }
    }

    pub fn new_multi_specification(name: T, specifications: Vec<(Specification, usize)>) -> Self {
        Self {
            name,
            specifications,
        }
    }

    pub(crate) fn name(&self) -> &T {
        &self.name
    }

    pub(crate) fn specifications(&self) -> &[(Specification, usize)] {
        &self.specifications
    }
}
