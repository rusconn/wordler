use std::{collections::HashSet, fmt};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    word: &'a str,
    chars: HashSet<char>,
}

impl<'a> From<&'a str> for Word<'a> {
    fn from(word: &'a str) -> Self {
        Self {
            word,
            chars: HashSet::from_iter(word.chars()),
        }
    }
}

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> Word<'a> {
    pub fn unique_letters(&self) -> impl Iterator<Item = &char> {
        self.chars.iter()
    }

    pub fn is_match(
        &self,
        regex: &Regex,
        contains: &HashSet<char>,
        not_contains: &HashSet<char>,
    ) -> bool {
        regex.is_match(self.word)
            && self.chars.is_superset(contains)
            && self.chars.is_disjoint(not_contains)
    }
}
