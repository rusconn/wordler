use crate::letter::Letter;

use super::LetterSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IncludesDerivative;

pub type Includes = LetterSet<IncludesDerivative>;

impl Default for Includes {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl Includes {
    pub fn is_subset<U>(&self, other: &LetterSet<U>) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn insert(&mut self, letter: Letter) {
        self.0.insert(letter);
    }
}
