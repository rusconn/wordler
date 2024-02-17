use rustc_hash::FxHashSet;

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Includes(FxHashSet<Letter>);

impl Includes {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn is_subset(&self, other: &FxHashSet<Letter>) -> bool {
        self.0.is_subset(other)
    }

    pub fn insert(&mut self, letter: Letter) -> bool {
        self.0.insert(letter)
    }
}
