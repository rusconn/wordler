use std::ops::Deref;

use derive_more::derive::{Deref, DerefMut};

use super::{Letter, LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct WordLetters(LetterSet);

impl WordLetters {
    pub fn from_unchecked(str: &str) -> Self {
        Self(Set::from_iter(str.bytes().map(Letter::from_unchecked)))
    }

    pub fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.deref().iter().copied()
    }
}
