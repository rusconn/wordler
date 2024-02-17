use std::{collections::HashSet, fmt};

use regex::Regex;

use crate::Letter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    word: &'a str,
    chars: HashSet<Letter>,
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
            chars: HashSet::from_iter(str.chars().map(Letter::unsafe_from)),
        }
    }

    pub fn unique_letters(&self) -> impl Iterator<Item = &Letter> {
        self.chars.iter()
    }

    pub fn is_match(
        &self,
        regex: &Regex,
        contains: &HashSet<Letter>,
        not_contains: &HashSet<Letter>,
    ) -> bool {
        regex.is_match(self.word)
            && self.chars.is_superset(contains)
            && self.chars.is_disjoint(not_contains)
    }
}
