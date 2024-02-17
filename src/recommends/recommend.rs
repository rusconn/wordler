use std::{cmp::Ordering, collections::HashMap, fmt};

use crate::{letter::Letter, Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommend<'a> {
    word: Word<'a>,
    score: i32,
}

impl<'a> fmt::Display for Recommend<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> PartialOrd for Recommend<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Recommend<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score - other.score {
            n if n < 0 => Ordering::Less,
            n if n > 0 => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl<'a> Recommend<'a> {
    /// Make sure the str is valid
    pub fn unsafe_from(str: &'a str) -> Self {
        Self {
            word: Word::unsafe_from(str),
            score: 0,
        }
    }

    pub fn update(&mut self, unused_letter_histogram: &HashMap<Letter, i32>) {
        self.score = self
            .word
            .unique_letters()
            .map(|c| unused_letter_histogram.get(c).unwrap_or(&0))
            .sum()
    }

    pub fn is_useful(&self) -> bool {
        self.score > 0
    }
}
