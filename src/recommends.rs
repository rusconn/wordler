mod recommend;

use std::collections::HashMap;

use itertools::Itertools;

use crate::{Candidates, Veileds};

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

    pub fn update(&mut self, candidates: &Candidates<'a>, veileds: &Veileds) {
        let mut veiled_letter_histogram = HashMap::default();

        for word in candidates.iter() {
            for letter in word.unique_letters() {
                if veileds.contains(letter) {
                    *veiled_letter_histogram.entry(*letter).or_insert(0) += 1;
                }
            }
        }

        for recommend in &mut self.0 {
            recommend.update(&veiled_letter_histogram);
        }

        self.0.retain(Recommend::is_useful);
        self.0.sort_unstable_by(|x, y| y.cmp(x));
    }

    pub fn print(&self) {
        if self.0.is_empty() {
            println!("Recommend: -");
        } else {
            println!("Recommend: [{}]", self.0.iter().take(5).join(","));
        }
    }
}
