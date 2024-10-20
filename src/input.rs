mod guess;
mod hints;
mod letter_info;
mod util;

use std::{
    io::{Stdin, Stdout},
    iter,
};

use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashSet;

use crate::{letter::Letter, word::Word};

use self::guess::Guess;
use self::hints::{Hint, Hints};
use self::letter_info::LetterInfo;

#[derive(Debug, Clone)]
pub(crate) struct Input {
    infos: Vec<LetterInfo>,
    includes: FxHashSet<Letter>,
    excludes: FxHashSet<Letter>,
    veileds: FxHashSet<Letter>,
    regex: Regex,
}

impl Input {
    pub(crate) fn default() -> Self {
        let infos: Vec<_> = iter::repeat_n(LetterInfo::default(), 5).collect();
        let regex = Self::regex(&infos);

        Self {
            infos,
            includes: Default::default(),
            excludes: Default::default(),
            veileds: (b'A'..=b'Z').map(Letter::from_unchecked).collect(),
            regex,
        }
    }

    fn regex(infos: &[LetterInfo]) -> Regex {
        Regex::new(Self::regex_string(infos).as_str())
            .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"))
    }

    fn regex_string(infos: &[LetterInfo]) -> String {
        infos.iter().map(LetterInfo::regex_string).join("")
    }

    pub(crate) fn read(&mut self, stdin: &Stdin, stdout: &mut Stdout) {
        let guess = Guess::read(stdin, stdout);
        let hints = Hints::read(stdin, stdout);
        self.update(guess, hints);
    }

    fn update(&mut self, guess: Guess, hints: Hints) {
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

        self.regex = Self::regex(&self.infos);
    }

    pub(crate) fn is_veiled(&self, letter: Letter) -> bool {
        self.veileds.contains(&letter)
    }

    pub(crate) fn is_match(&self, Word { str, letters }: &Word) -> bool {
        self.regex.is_match(str)
            && self.includes.is_subset(letters)
            && self.excludes.is_disjoint(letters)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use super::*;

    fn letters(bytes: &[u8]) -> FxHashSet<Letter> {
        bytes.iter().copied().map(Letter::from_unchecked).collect()
    }

    fn complement(l: &FxHashSet<Letter>) -> FxHashSet<Letter> {
        let u = LazyCell::new(|| {
            let v: Vec<_> = (b'A'..=b'Z').collect();
            letters(v.as_slice())
        });
        u.difference(l).copied().collect()
    }

    #[test]
    fn update() {
        let mut input = Input::default();
        assert_eq!(input.includes, letters(b""));
        assert_eq!(input.excludes, letters(b""));
        assert_eq!(input.veileds, complement(&letters(b"")));
        assert_eq!(Input::regex_string(&input.infos), ".....");

        let guess = Guess::try_from("SERIA").unwrap();
        let hints = Hints::try_from("10100").unwrap();
        input.update(guess, hints);
        assert_eq!(input.includes, letters(b"SR"));
        assert_eq!(input.excludes, letters(b"EIA"));
        assert_eq!(input.veileds, complement(&letters(b"SERIA")));
        assert_eq!(Input::regex_string(&input.infos), "[^S][^E][^R][^I][^A]");

        let guess = Guess::try_from("HYSON").unwrap();
        let hints = Hints::try_from("01200").unwrap();
        input.update(guess, hints);
        assert_eq!(input.includes, letters(b"SRYS"));
        assert_eq!(input.excludes, letters(b"EIAHON"));
        assert_eq!(input.veileds, complement(&letters(b"SERIAHYSON")));
        assert_eq!(Input::regex_string(&input.infos), "[^HS][^EY]S[^IO][^AN]");
    }
}
