mod guess;
mod hints;
mod letter_info;
mod util;

use std::{
    io::{Stdin, Stdout},
    iter,
};

use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::{letter::Letter, word::Letters};

use self::guess::Guess;
use self::hints::{Hint, Hints};
use self::letter_info::LetterInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Input {
    infos: Vec<LetterInfo>,
    includes: FxHashSet<Letter>,
    excludes: FxHashSet<Letter>,
    veileds: FxHashSet<Letter>,
}

impl Input {
    pub(crate) fn default() -> Self {
        Self {
            infos: iter::repeat_n(LetterInfo::default(), 5).collect(),
            includes: Default::default(),
            excludes: Default::default(),
            veileds: (b'A'..=b'Z').map(Letter::from_unchecked).collect(),
        }
    }

    pub(crate) fn read(&mut self, stdin: &Stdin, stdout: &mut Stdout) {
        let guess = Guess::read(stdin, stdout);
        let hints = Hints::read(stdin, stdout);

        for ((letter, hint), info) in guess.iter().zip(hints.iter()).zip(self.infos.iter_mut()) {
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
        let mut input = Input::default();
        assert_eq!(input.as_regex(), ".....");

        input.infos[1].not(Letter::from_unchecked(b'A'));
        assert_eq!(input.as_regex(), ".[^A]...");

        input.infos[4].not(Letter::from_unchecked(b'B'));
        assert_eq!(input.as_regex(), ".[^A]..[^B]");

        input.infos[1].not(Letter::from_unchecked(b'C'));
        assert!([".[^AC]..[^B]", ".[^CA]..[^B]"].contains(&input.as_regex().as_str()));

        input.infos[2].correct(Letter::from_unchecked(b'D'));
        assert!([".[^AC]D.[^B]", ".[^CA]D.[^B]"].contains(&input.as_regex().as_str()));

        input.infos[1].correct(Letter::from_unchecked(b'E'));
        assert_eq!(input.as_regex(), ".ED.[^B]");
    }
}
