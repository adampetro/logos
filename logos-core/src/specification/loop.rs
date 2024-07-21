use crate::Specification;

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

    pub(crate) fn min(&self) -> usize {
        self.min
    }

    pub(crate) fn max(&self) -> Option<usize> {
        self.max
    }

    pub(crate) fn specification(&self) -> &Specification {
        &self.specification
    }
}
