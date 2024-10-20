mod recommend;

use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::{candidates::Candidates, dict::WORDS, input::Input, letter::Letter};

use self::recommend::Recommend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Recommends<'a>(Vec<Recommend<'a>>);

type VeiledLetterHistogram = FxHashMap<Letter, i32>;

impl<'a> Default for Recommends<'a> {
    fn default() -> Self {
        Self(WORDS.into_iter().map(Recommend::from_unchecked).collect())
    }
}

impl<'a> Recommends<'a> {
    pub(crate) fn update(&mut self, candidates: &Candidates<'a>, input: &Input) {
        let mut histogram: VeiledLetterHistogram = Default::default();

        for word in candidates.iter() {
            for letter in word.unique_letters() {
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

    pub(crate) fn print(&self) {
        if self.0.is_empty() {
            println!("Recommend: -");
        } else {
            println!("Recommend: [{}]", self.0.iter().take(5).join(","));
        }
    }
}
