use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use crate::{candidates::Candidates, word::Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommends<'a> {
    words: Vec<Word<'a>>,
}

impl<'a, const N: usize> From<&[&'a str; N]> for Recommends<'a> {
    fn from(words: &[&'a str; N]) -> Self {
        Self {
            words: words.iter().map(|&word| Word::from(word)).collect(),
        }
    }
}

impl<'a> Recommends<'a> {
    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    pub fn take(&self, n: usize) -> impl Iterator<Item = &Word<'a>> {
        self.words.iter().take(n)
    }

    pub fn update(&mut self, candidates: &Candidates<'a>, unuseds: &HashSet<char>) {
        let mut unused_letter_histogram = HashMap::default();

        for word in candidates.iter() {
            for letter in word.unique_letters() {
                if unuseds.contains(letter) {
                    *unused_letter_histogram.entry(*letter).or_insert(0) += 1;
                }
            }
        }

        self.words
            .sort_unstable_by_key(|word| Reverse(word.score(&unused_letter_histogram)));

        if self.words[0].score(&unused_letter_histogram) == 0 {
            self.words.clear();
        }
    }
}
