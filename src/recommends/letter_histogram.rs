use std::collections::{hash_map::Entry, HashMap};

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LetterHistogram(HashMap<Letter, i32>);

impl LetterHistogram {
    pub fn get(&self, letter: &Letter) -> Option<&i32> {
        self.0.get(letter)
    }

    pub fn entry(&mut self, letter: Letter) -> Entry<'_, Letter, i32> {
        self.0.entry(letter)
    }
}