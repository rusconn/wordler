use std::{collections::HashMap, fmt};

use crate::word::Word;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommend<'a> {
    word: Word<'a>,
}

impl<'a> From<&'a str> for Recommend<'a> {
    fn from(word: &'a str) -> Self {
        Self {
            word: Word::from(word),
        }
    }
}

impl<'a> fmt::Display for Recommend<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> Recommend<'a> {
    pub fn score(&self, unused_letter_histogram: &HashMap<char, i32>) -> i32 {
        self.word
            .unique_letters()
            .map(|c| unused_letter_histogram.get(&c).unwrap_or(&0))
            .sum()
    }

    pub fn is_useless(&self, unused_letter_histogram: &HashMap<char, i32>) -> bool {
        self.score(unused_letter_histogram) == 0
    }
}
