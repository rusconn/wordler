use std::collections::HashSet;

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Excludes(HashSet<Letter>);

impl Excludes {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn is_disjoint(&self, other: &HashSet<Letter>) -> bool {
        self.0.is_disjoint(other)
    }

    pub fn insert(&mut self, letter: Letter) -> bool {
        self.0.insert(letter)
    }
}
