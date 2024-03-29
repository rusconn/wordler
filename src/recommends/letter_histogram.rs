use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

use crate::letter::Letter;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LetterHistogram(FxHashMap<Letter, i32>);

impl LetterHistogram {
    pub fn get(&self, letter: Letter) -> Option<i32> {
        self.0.get(&letter).copied()
    }

    pub fn entry(&mut self, letter: Letter) -> Entry<'_, Letter, i32> {
        self.0.entry(letter)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Letter, &mut i32)> {
        self.0.iter_mut()
    }
}
