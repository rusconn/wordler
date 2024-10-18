use std::ops::{Deref, DerefMut};

use super::{Letter, LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordLetters(LetterSet);

impl Deref for WordLetters {
    type Target = LetterSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WordLetters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WordLetters {
    pub fn from_unchecked(str: &str) -> Self {
        Self(Set::from_iter(str.bytes().map(Letter::from_unchecked)))
    }

    pub fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.deref().iter().copied()
    }
}
