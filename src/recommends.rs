use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use crate::{candidates::Candidates, recommend::Recommend};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommends<'a> {
    recommends: Vec<Recommend<'a>>,
}

impl<'a, const N: usize> From<&[&'a str; N]> for Recommends<'a> {
    fn from(words: &[&'a str; N]) -> Self {
        Self {
            recommends: words.iter().map(|&word| Recommend::from(word)).collect(),
        }
    }
}

impl<'a> Recommends<'a> {
    pub fn is_empty(&self) -> bool {
        self.recommends.is_empty()
    }

    pub fn take(&self, n: usize) -> impl Iterator<Item = &Recommend<'a>> {
        self.recommends.iter().take(n)
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

        self.recommends
            .sort_unstable_by_key(|recommend| Reverse(recommend.score(&unused_letter_histogram)));

        let top_recommend = &self.recommends[0];

        if top_recommend.is_useless(&unused_letter_histogram) {
            self.recommends.clear();
        }
    }
}
