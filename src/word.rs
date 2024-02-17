use std::{collections::HashSet, fmt};

use regex::Regex;

use crate::{Includes, Letter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    word: &'a str,
    letter_set: HashSet<Letter>,
}

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> Word<'a> {
    /// Make sure the str is valid
    pub fn unsafe_from(str: &'a str) -> Self {
        Self {
            word: str,
            letter_set: HashSet::from_iter(str.chars().map(Letter::unsafe_from)),
        }
    }

    pub fn unique_letters(&self) -> impl Iterator<Item = &Letter> {
        self.letter_set.iter()
    }

    pub fn is_match(
        &self,
        regex: &Regex,
        includes: &Includes,
        not_contains: &HashSet<Letter>,
    ) -> bool {
        regex.is_match(self.word)
            && includes.is_subset(&self.letter_set)
            && self.letter_set.is_disjoint(not_contains)
    }
}
