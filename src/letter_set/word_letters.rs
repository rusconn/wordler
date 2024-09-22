use crate::letter::Letter;

use super::{LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WordDerivative;

pub type WordLetters = LetterSet<WordDerivative>;

impl WordLetters {
    pub fn from_unchecked(str: &str) -> Self {
        Self(
            Set::from_iter(str.bytes().map(Letter::from_unchecked)),
            Default::default(),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.0.iter().copied()
    }
}
