mod recommend;

use std::collections::{HashMap, HashSet};

use crate::{Candidates, Letter};

use self::recommend::Recommend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommends<'a>(Vec<Recommend<'a>>);

impl<'a> Recommends<'a> {
    /// Make sure the strs are valid
    pub fn unsafe_from(strs: &[&'a str]) -> Self {
        Self(
            strs.iter()
                .map(|&str| Recommend::unsafe_from(str))
                .collect(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn take(&self, n: usize) -> impl Iterator<Item = &Recommend<'a>> {
        self.0.iter().take(n)
    }

    pub fn update(&mut self, candidates: &Candidates<'a>, unuseds: &HashSet<Letter>) {
        let mut unused_letter_histogram = HashMap::default();

        for word in candidates.iter() {
            for letter in word.unique_letters() {
                if unuseds.contains(letter) {
                    *unused_letter_histogram.entry(*letter).or_insert(0) += 1;
                }
            }
        }

        for recommend in &mut self.0 {
            recommend.update(&unused_letter_histogram);
        }

        self.0.retain(Recommend::is_useful);
        self.0.sort_unstable_by(|x, y| y.cmp(x));
    }
}
