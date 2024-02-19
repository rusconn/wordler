use crate::Letter;

use super::LetterSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExcludesDerivative;

pub type Excludes = LetterSet<ExcludesDerivative>;

impl Default for Excludes {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl Excludes {
    pub fn is_disjoint<U>(&self, other: &LetterSet<U>) -> bool {
        self.0.is_disjoint(&other.0)
    }

    pub fn insert(&mut self, letter: Letter) {
        self.0.insert(letter);
    }
}
