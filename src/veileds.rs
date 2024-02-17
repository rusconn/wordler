use rustc_hash::FxHashSet;

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Veileds(FxHashSet<Letter>);

impl Veileds {
    /// Make sure the bytes are valid
    pub fn unsafe_from<T: Iterator<Item = u8>>(bytes: T) -> Self {
        Self(FxHashSet::from_iter(bytes.map(Letter::unsafe_from)))
    }

    pub fn contains(&self, letter: Letter) -> bool {
        self.0.contains(&letter)
    }

    pub fn remove(&mut self, letter: Letter) -> bool {
        self.0.remove(&letter)
    }
}
