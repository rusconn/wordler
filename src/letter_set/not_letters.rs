use crate::letter::Letter;

use super::{LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LetterInfoNotDerivative;

pub type NotLetters = LetterSet<LetterInfoNotDerivative>;

impl NotLetters {
    pub fn new(letter: Letter) -> Self {
        Self(Set::from_iter([letter]), Default::default())
    }

    pub fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.0.iter().copied()
    }

    pub fn insert(&mut self, letter: Letter) {
        self.0.insert(letter);
    }
}
