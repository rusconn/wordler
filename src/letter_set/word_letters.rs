use crate::Letter;

use super::{LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WordDerivative;

pub type WordLetters = LetterSet<WordDerivative>;

impl From<&str> for WordLetters {
    fn from(str: &str) -> Self {
        Self(
            Set::from_iter(str.bytes().map(Letter::unsafe_from)),
            Default::default(),
        )
    }
}

impl WordLetters {
    pub fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.0.iter().copied()
    }
}
