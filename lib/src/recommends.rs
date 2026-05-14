mod recommend;

use rustc_hash::FxHashMap;

use crate::{State, dict::WORDS, letter::Letter};

use recommend::Recommend;

#[derive(Debug, PartialEq, Eq)]
pub struct Recommends(Vec<Recommend>);

type VeiledLetterHistogram = FxHashMap<Letter, i32>;

impl Recommends {
    pub fn new(state: &State) -> Self {
        let mut recommends = Self(WORDS.iter().map(Recommend::new).collect());
        recommends.update(state);
        recommends
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Recommend> {
        self.0.iter()
    }

    pub fn update(&mut self, state: &State) {
        let mut histogram: VeiledLetterHistogram = Default::default();

        for word in state.candidates().iter() {
            for letter in word.as_letter_set() {
                if state.veileds().contains(letter) {
                    *histogram.entry(*letter).or_insert(0) += 1;
                }
            }
        }

        // common letters must not be scored
        for (_, n) in histogram.iter_mut() {
            if *n as usize == state.candidates().len() {
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
