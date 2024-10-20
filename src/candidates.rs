use itertools::Itertools;

use crate::{dict::WORDS, input::Input, word::Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Candidates<'a>(Vec<Word<'a>>);

impl<'a> Default for Candidates<'a> {
    fn default() -> Self {
        Self(WORDS.into_iter().map(Word::from_unchecked).collect())
    }
}

impl<'a> Candidates<'a> {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Word<'a>> {
        self.0.iter()
    }

    /// returns true if game over
    pub(crate) fn print(&self) -> bool {
        match self.0.len() {
            0 => println!("Woops, there are no more words"),
            1 => println!("Found: {}", self.0[0]),
            n if n <= 50 => println!("Remaining: [{}]", self.0.iter().join(",")),
            n => println!("Remaining: Too many, didn't print: {n}"),
        }

        self.0.len() <= 1
    }

    pub(crate) fn retain(&mut self, input: &Input) {
        self.0.retain(|word| input.is_match(word));
    }
}
