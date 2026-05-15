use rustc_hash::FxHashSet;

use crate::{Word, letter::Letter};

#[cfg_attr(test, derive(Clone, PartialEq, Eq))]
#[derive(Debug)]
pub(crate) struct Veileds(FxHashSet<Letter>);

impl Default for Veileds {
    fn default() -> Self {
        Self((b'A'..=b'Z').map(Letter::from_unchecked).collect())
    }
}

impl Veileds {
    pub(crate) fn contains(&self, letter: &Letter) -> bool {
        self.0.contains(letter)
    }

    pub(crate) fn unveil(&mut self, word: &Word) {
        let word_letters = word.as_letter_set();
        self.0.retain(|letter| !word_letters.contains(letter));
    }
}
