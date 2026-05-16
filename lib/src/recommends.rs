mod dict;
mod recommend;

use crate::{State, Word, dict::WORDS, letter_map::LetterMap, letter_set::LetterSet};

use recommend::Recommend;

#[derive(Debug, PartialEq, Eq)]
pub struct Recommends(Vec<Recommend>);

type LetterHistogram = LetterMap<i32>;

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
        let mut histogram = LetterHistogram::default();

        for word in state.candidates().iter() {
            for letter in word.as_letter_set().letters() {
                if state.constraints().is_veiled(letter) {
                    *histogram.get_mut(letter) += 1;
                }
            }
        }

        for recommend in self.0.iter_mut() {
            recommend.update(&histogram, state.candidates().len() as i32);
        }

        self.0.retain(Recommend::is_useful);
        self.0.sort_unstable_by(|x, y| y.cmp(x));
    }
}

impl Word {
    fn as_letter_set(&self) -> LetterSet {
        dict::WORD_LETTER_SETS[self.as_index()]
    }
}
