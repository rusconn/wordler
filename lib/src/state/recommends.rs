mod recommend;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{dict::WORDS, letter::Letter};

use super::candidates::Candidates;

use self::recommend::Recommend;

#[derive(Debug, PartialEq, Eq)]
pub struct Recommends<'a>(Vec<Recommend<'a>>);

type VeiledLetterHistogram = FxHashMap<Letter, i32>;

impl Default for Recommends<'_> {
    fn default() -> Self {
        Self(WORDS.into_iter().map(Recommend::from_unchecked).collect())
    }
}

impl<'a> Recommends<'a> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Recommend<'a>> {
        self.0.iter()
    }

    pub(crate) fn update(&mut self, candidates: &Candidates<'a>, veileds: &FxHashSet<Letter>) {
        let mut histogram: VeiledLetterHistogram = Default::default();

        for word in candidates.iter() {
            for &letter in word.letters.iter() {
                if veileds.contains(&letter) {
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
