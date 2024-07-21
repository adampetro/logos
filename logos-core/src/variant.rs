use crate::Specification;

pub struct Variant {
    name: String,
    priority: usize,
    specification: Specification,
}

impl Variant {
    pub fn new(name: String, specification: Specification, priority: Option<usize>) -> Self {
        Self {
            name,
            priority: priority.unwrap_or_else(|| specification.default_priority()),
            specification,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn priority(&self) -> usize {
        self.priority
    }

    pub(crate) fn specification(&self) -> &Specification {
        &self.specification
    }
}
