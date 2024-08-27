use crate::Specification;

#[derive(PartialEq, Clone)]
pub struct Loop {
    min: usize,
    max: Option<usize>,
    specification: Box<Specification>,
}

impl Loop {
    pub(crate) fn default_priority(&self) -> usize {
        self.min * self.specification.default_priority()
    }

    pub(crate) fn new(min: usize, max: Option<usize>, specification: Specification) -> Self {
        Self {
            min,
            max,
            specification: Box::new(specification),
        }
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }

    pub fn specification(&self) -> &Specification {
        &self.specification
    }
}
