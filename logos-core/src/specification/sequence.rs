use crate::Specification;

pub struct Sequence {
    specifications: Vec<Specification>,
}

impl Sequence {
    pub(crate) fn default_priority(&self) -> usize {
        self.specifications
            .iter()
            .map(Specification::default_priority)
            .sum()
    }

    pub(crate) fn new_str_sequence(s: &str) -> Self {
        Self {
            specifications: s
                .as_bytes()
                .iter()
                .map(|b| Specification::Byte(*b))
                .collect(),
        }
    }

    pub(crate) fn iter(&self) -> impl DoubleEndedIterator<Item = &Specification> {
        self.specifications.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.specifications.len()
    }

    pub(crate) fn new(specifications: Vec<Specification>) -> Self {
        Self { specifications }
    }
}
