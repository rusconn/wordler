use std::fmt;

use regex::Regex;
use rustc_hash::FxHashSet;

use crate::{Excludes, Includes, Letter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    word: &'a str,
    letter_set: FxHashSet<Letter>,
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
            letter_set: FxHashSet::from_iter(str.chars().map(Letter::unsafe_from)),
        }
    }

    pub fn unique_letters(&self) -> impl Iterator<Item = Letter> + '_ {
        self.letter_set.iter().copied()
    }

    pub fn is_match(&self, regex: &Regex, includes: &Includes, excludes: &Excludes) -> bool {
        regex.is_match(self.word)
            && includes.is_subset(&self.letter_set)
            && excludes.is_disjoint(&self.letter_set)
    }
}
