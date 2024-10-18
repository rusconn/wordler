use std::ops::{Deref, DerefMut};

use super::{Letter, LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Veileds(LetterSet);

impl Default for Veileds {
    fn default() -> Self {
        Self(Set::from_iter((b'A'..=b'Z').map(Letter::from_unchecked)))
    }
}

impl Deref for Veileds {
    type Target = LetterSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Veileds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
