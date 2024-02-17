use std::collections::HashSet;

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Veileds(HashSet<Letter>);

impl Veileds {
    /// Make sure the chars are valid
    pub fn unsafe_from<T: Iterator<Item = char>>(chars: T) -> Self {
        Self(HashSet::from_iter(chars.map(Letter::unsafe_from)))
    }

    pub fn contains(&self, letter: &Letter) -> bool {
        self.0.contains(letter)
    }

    pub fn remove(&mut self, letter: &Letter) -> bool {
        self.0.remove(letter)
    }
}
