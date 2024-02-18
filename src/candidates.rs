use std::ops::Index;

use itertools::Itertools;
use regex::Regex;

use crate::{Excludes, Includes, LetterInfos, Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates<'a>(Vec<Word<'a>>);

impl<'a> Index<usize> for Candidates<'a> {
    type Output = Word<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> Candidates<'a> {
    /// Make sure the strs are valid
    pub fn unsafe_from<const N: usize>(strs: [&'a str; N]) -> Self {
        Self(strs.into_iter().map(Word::unsafe_from).collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Word<'a>> {
        self.0.iter()
    }

    /// returns true if game over
    pub fn print(&self) -> bool {
        match self.0.len() {
            0 => println!("Woops, there are no more words"),
            1 => println!("Found: {}", self.0[0]),
            n if n <= 50 => println!("Remaining: [{}]", self.0.iter().join(",")),
            n => println!("Remaining: Too many, didn't print: {n}"),
        }

        self.0.len() <= 1
    }

    pub fn retain(&mut self, letter_infos: &LetterInfos, includes: &Includes, excludes: &Excludes) {
        let regex = Regex::new(&letter_infos.as_regex())
            .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"));

        self.0
            .retain(|word| word.is_match(&regex, includes, excludes));
    }
}
