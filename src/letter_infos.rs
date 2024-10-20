mod letter_info;

use std::iter;

use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::{
    input::{Hint, Input},
    letter::Letter,
    word::Letters,
};

use self::letter_info::LetterInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LetterInfos {
    infos: Vec<LetterInfo>,
    includes: FxHashSet<Letter>,
    excludes: FxHashSet<Letter>,
    veileds: FxHashSet<Letter>,
}

impl Default for LetterInfos {
    fn default() -> Self {
        Self {
            infos: iter::repeat_n(LetterInfo::default(), 5).collect(),
            includes: Default::default(),
            excludes: Default::default(),
            veileds: (b'A'..=b'Z').map(Letter::from_unchecked).collect(),
        }
    }
}

impl LetterInfos {
    pub(crate) fn apply(&mut self, input: Input) {
        for ((letter, hint), info) in input.iter().zip(self.infos.iter_mut()) {
            match hint {
                Hint::NotExists => {
                    info.not(letter);
                    self.excludes.insert(letter);
                }
                Hint::WrongSpot => {
                    info.not(letter);
                    self.includes.insert(letter);
                }
                Hint::CorrectSpot => {
                    info.correct(letter);
                    self.includes.insert(letter);
                }
            }

            self.veileds.remove(&letter);
        }
    }

    pub(crate) fn is_veiled(&self, letter: Letter) -> bool {
        self.veileds.contains(&letter)
    }

    pub(crate) fn is_match(&self, letters: &Letters) -> bool {
        self.includes.is_subset(letters) && self.excludes.is_disjoint(letters)
    }

    pub(crate) fn as_regex(&self) -> String {
        self.infos.iter().map(LetterInfo::as_regex).join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::letter::Letter;

    use super::*;

    #[test]
    fn operations() {
        let mut letter_infos = LetterInfos::default();
        assert_eq!(letter_infos.as_regex(), ".....");

        letter_infos.infos[1].not(Letter::from_unchecked(b'A'));
        assert_eq!(letter_infos.as_regex(), ".[^A]...");

        letter_infos.infos[4].not(Letter::from_unchecked(b'B'));
        assert_eq!(letter_infos.as_regex(), ".[^A]..[^B]");

        letter_infos.infos[1].not(Letter::from_unchecked(b'C'));
        assert!([".[^AC]..[^B]", ".[^CA]..[^B]"].contains(&letter_infos.as_regex().as_str()));

        letter_infos.infos[2].correct(Letter::from_unchecked(b'D'));
        assert!([".[^AC]D.[^B]", ".[^CA]D.[^B]"].contains(&letter_infos.as_regex().as_str()));

        letter_infos.infos[1].correct(Letter::from_unchecked(b'E'));
        assert_eq!(letter_infos.as_regex(), ".ED.[^B]");
    }
}
