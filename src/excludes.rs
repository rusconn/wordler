use rustc_hash::FxHashSet;

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Excludes(FxHashSet<Letter>);

impl Excludes {
    pub fn is_disjoint(&self, other: &FxHashSet<Letter>) -> bool {
        self.0.is_disjoint(other)
    }

    pub fn insert(&mut self, letter: Letter) -> bool {
        self.0.insert(letter)
    }
}
