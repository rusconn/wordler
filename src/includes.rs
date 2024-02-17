use std::collections::HashSet;

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Includes(HashSet<Letter>);

impl Includes {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn is_subset(&self, other: &HashSet<Letter>) -> bool {
        self.0.is_subset(other)
    }

    pub fn insert(&mut self, letter: Letter) -> bool {
        self.0.insert(letter)
    }
}
