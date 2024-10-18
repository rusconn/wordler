use crate::letter::Letter;

use super::{LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordDerivative;

pub type WordLetters = LetterSet<WordDerivative>;

impl WordLetters {
    pub fn from_unchecked(str: &str) -> Self {
        Self(
            Set::from_iter(str.bytes().map(Letter::from_unchecked)),
            Default::default(),
        )
    }
}
