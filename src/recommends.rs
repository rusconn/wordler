mod recommend;

use std::fmt;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::{candidates::Candidates, dict::WORDS, input::Input, letter::Letter};

use self::recommend::Recommend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommends<'a>(Vec<Recommend<'a>>);

type VeiledLetterHistogram = FxHashMap<Letter, i32>;

impl Default for Recommends<'_> {
    fn default() -> Self {
        Self(WORDS.into_iter().map(Recommend::from_unchecked).collect())
    }
}

impl fmt::Display for Recommends<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "Recommend: -")
        } else {
            write!(f, "Recommend: [{}]", self.0.iter().take(5).join(","))
        }
    }
}

impl<'a> Recommends<'a> {
    pub fn update(&mut self, candidates: &Candidates<'a>, input: &Input) {
        let mut histogram: VeiledLetterHistogram = Default::default();

        for word in candidates.iter() {
            for &letter in word.letters.iter() {
                if input.is_veiled(letter) {
                    *histogram.entry(letter).or_insert(0) += 1;
                }
            }
        }

        // common letters must not be scored
        for (_, n) in histogram.iter_mut() {
            if *n as usize == candidates.len() {
                *n = 0;
            }
        }

        for recommend in self.0.iter_mut() {
            recommend.update(&histogram);
        }

        self.0.retain(Recommend::is_useful);
        self.0.sort_unstable_by(|x, y| y.cmp(x));
    }
}
