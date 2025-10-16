mod candidates;
mod letter_info;
mod recommends;

use std::iter;

use rustc_hash::FxHashSet;

use crate::{
    input::{
        guess::Guess,
        hints::{Hints, hint::Hint},
    },
    letter::Letter,
};

use self::letter_info::LetterInfo;

pub use self::{candidates::Candidates, recommends::Recommends};

#[derive(Debug, Clone)]
pub struct State<'a> {
    infos: Vec<LetterInfo>,
    includes: FxHashSet<Letter>,
    excludes: FxHashSet<Letter>,
    veileds: FxHashSet<Letter>,
    candidates: Candidates<'a>,
    recommends: Recommends<'a>,
}

impl Default for State<'_> {
    fn default() -> Self {
        let veileds = (b'A'..=b'Z').map(Letter::from_unchecked).collect();
        let candidates = Candidates::default();
        let mut recommends = Recommends::default();

        recommends.update(&candidates, &veileds);

        Self {
            infos: iter::repeat_n(LetterInfo::default(), 5).collect::<Vec<_>>(),
            includes: Default::default(),
            excludes: Default::default(),
            veileds,
            candidates,
            recommends,
        }
    }
}

impl State<'_> {
    pub fn update(&mut self, guess: Guess, hints: Hints) {
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

        self.recommends.update(&self.candidates, &self.veileds);
        self.candidates
            .retain(&self.infos, &self.includes, &self.excludes);
    }

    pub fn candidates(&self) -> &Candidates {
        &self.candidates
    }

    pub fn recommends(&self) -> &Recommends {
        &self.recommends
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    static U: LazyLock<FxHashSet<Letter>> =
        LazyLock::new(|| letters(&(b'A'..=b'Z').collect::<Vec<_>>()));

    fn letters(bytes: &[u8]) -> FxHashSet<Letter> {
        bytes.iter().copied().map(Letter::from_unchecked).collect()
    }

    fn complement(l: &FxHashSet<Letter>) -> FxHashSet<Letter> {
        U.difference(l).copied().collect()
    }

    #[test]
    fn update() {
        let mut state = State::default();
        assert_eq!(state.includes, letters(b""));
        assert_eq!(state.excludes, letters(b""));
        assert_eq!(state.veileds, complement(&letters(b"")));

        let guess = "SERIA".parse().unwrap();
        let hints = "10100".parse().unwrap();
        state.update(guess, hints);
        assert_eq!(state.includes, letters(b"SR"));
        assert_eq!(state.excludes, letters(b"EIA"));
        assert_eq!(state.veileds, complement(&letters(b"SERIA")));

        let guess = "HYSON".parse().unwrap();
        let hints = "01200".parse().unwrap();
        state.update(guess, hints);
        assert_eq!(state.includes, letters(b"SRYS"));
        assert_eq!(state.excludes, letters(b"EIAHON"));
        assert_eq!(state.veileds, complement(&letters(b"SERIAHYSON")));
    }
}
