use crate::Specification;

pub struct Variant<T> {
    name: T,
    priority: usize,
    specification: Specification,
}

impl<T> Variant<T> {
    pub fn new(name: T, specification: Specification, priority: Option<usize>) -> Self {
        Self {
            name,
            priority: priority.unwrap_or_else(|| specification.default_priority()),
            specification,
        }
    }

    pub(crate) fn name(&self) -> &T {
        &self.name
    }

    pub(crate) fn priority(&self) -> usize {
        self.priority
    }

    pub(crate) fn specification(&self) -> &Specification {
        &self.specification
    }
}
