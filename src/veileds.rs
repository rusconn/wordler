use rustc_hash::FxHashSet;

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Veileds(FxHashSet<Letter>);

impl Default for Veileds {
    fn default() -> Self {
        Self(FxHashSet::from_iter((b'A'..=b'Z').map(Letter::unsafe_from)))
    }
}

impl Veileds {
    pub fn contains(&self, letter: Letter) -> bool {
        self.0.contains(&letter)
    }

    pub fn remove(&mut self, letter: Letter) -> bool {
        self.0.remove(&letter)
    }
}
