use std::ops::{Deref, DerefMut};

use super::{Letter, LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotLetters(LetterSet);

impl Deref for NotLetters {
    type Target = LetterSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NotLetters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NotLetters {
    pub fn new(letter: Letter) -> Self {
        Self(Set::from_iter([letter]))
    }
}
