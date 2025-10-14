use std::fmt;

use itertools::Itertools;

use crate::{dict::WORDS, input::Input, word::Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates<'a>(Vec<Word<'a>>);

impl Default for Candidates<'_> {
    fn default() -> Self {
        Self(WORDS.into_iter().map(Word::from_unchecked).collect())
    }
}

impl fmt::Display for Candidates<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.len() {
            0 => write!(f, "Woops, there are no more words"),
            1 => write!(f, "Found: {}", self.0[0]),
            n if n <= 50 => write!(f, "Remaining: [{}]", self.0.iter().join(",")),
            n => write!(f, "Remaining: Too many, didn't print: {n}"),
        }
    }
}

impl<'a> Candidates<'a> {
    pub fn retain(&mut self, input: &Input) {
        self.0.retain(|word| input.is_match(word));
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Word<'a>> {
        self.0.iter()
    }
}
