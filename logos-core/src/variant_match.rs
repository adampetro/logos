use crate::Specification;

pub trait VariantMatch: std::fmt::Debug {
    fn priority(&self) -> usize;
    fn specification(&self) -> &Specification;
    fn is_same_variant(&self, other: &Self) -> bool;
}

pub struct SimpleVariantMatch<'a> {
    name: &'a str,
    specification: Specification,
    priority: usize,
}

impl std::fmt::Debug for SimpleVariantMatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleVariantMatch")
            .field("name", &self.name)
            .field("priority", &self.priority)
            .finish()
    }
}

impl<'a> VariantMatch for SimpleVariantMatch<'a> {
    fn priority(&self) -> usize {
        self.priority
    }

    fn specification(&self) -> &Specification {
        &self.specification
    }

    fn is_same_variant(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> SimpleVariantMatch<'a> {
    pub fn new(name: &'a str, specification: Specification, priority: Option<usize>) -> Self {
        let priority = priority.unwrap_or_else(|| specification.default_priority());
        Self {
            name,
            specification,
            priority,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}
