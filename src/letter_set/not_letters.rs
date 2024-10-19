use std::iter;

use derive_more::derive::{Deref, DerefMut};

use super::{Letter, LetterSet};

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct NotLetters(LetterSet);

impl NotLetters {
    pub fn new(letter: Letter) -> Self {
        Self(iter::once(letter).collect())
    }
}
