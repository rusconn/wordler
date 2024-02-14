use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    value: &'a str,
    chars: HashSet<char>,
}

impl<'a> From<&'a str> for Word<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            value,
            chars: HashSet::from_iter(value.chars()),
        }
    }
}

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a> Word<'a> {
    pub fn score(&self, unused_letter_histogram: &HashMap<char, i32>) -> i32 {
        self.chars
            .iter()
            .map(|c| unused_letter_histogram.get(&c).unwrap_or(&0))
            .sum()
    }

    pub fn unique_letters(&self) -> impl Iterator<Item = &char> {
        self.chars.iter()
    }

    pub fn is_match(
        &self,
        regex: &Regex,
        contain: &HashSet<char>,
        not_contain: &HashSet<char>,
    ) -> bool {
        regex.is_match(self.value)
            && self.chars.is_superset(contain)
            && self.chars.is_disjoint(not_contain)
    }
}
