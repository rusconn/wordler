mod letter_histogram;
mod recommend;

use itertools::Itertools;

use crate::{candidates::Candidates, dict::WORDS, letter_set::Veileds};

use self::{letter_histogram::LetterHistogram, recommend::Recommend};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommends<'a>(Vec<Recommend<'a>>);

impl<'a> Default for Recommends<'a> {
    fn default() -> Self {
        Self(WORDS.into_iter().map(Recommend::from_unchecked).collect())
    }
}

impl<'a> Recommends<'a> {
    pub fn update(&mut self, candidates: &Candidates<'a>, veileds: &Veileds) {
        let mut veiled_letter_histogram = LetterHistogram::default();

        for word in candidates.iter() {
            for letter in word.unique_letters() {
                if veileds.contains(letter) {
                    *veiled_letter_histogram.entry(letter).or_insert(0) += 1;
                }
            }
        }

        // common letters must not be scored
        for (_, n) in veiled_letter_histogram.iter_mut() {
            if *n as usize == candidates.len() {
                *n = 0;
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
