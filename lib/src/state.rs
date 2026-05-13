mod candidates;
mod position_constraint;
mod to_regex_string;

use std::iter;

use rustc_hash::FxHashSet;
use thiserror::Error;

use crate::{letter::Letter, word::Word};

use super::hints::{Hint, Hints};

use position_constraint::PositionConstraint;

pub use {candidates::Candidates, position_constraint::CheckHintError as InvalidHintError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    constraints: Vec<PositionConstraint>,
    includes: FxHashSet<Letter>,
    excludes: FxHashSet<Letter>,
    pub(crate) veileds: FxHashSet<Letter>,
    pub(crate) candidates: Candidates,
}

impl Default for State {
    fn default() -> Self {
        let veileds = (b'A'..=b'Z').map(Letter::from_unchecked).collect();
        let candidates = Candidates::default();

        Self {
            constraints: iter::repeat_n(PositionConstraint::default(), 5).collect::<Vec<_>>(),
            includes: Default::default(),
            excludes: Default::default(),
            veileds,
            candidates,
        }
    }
}

impl State {
    pub fn update(&mut self, guess: &Word, hints: &Hints) -> Result<(), UpdateError> {
        for ((letter, hint), constraint) in guess
            .iter() //
            .zip(hints.iter())
            .zip(self.constraints.iter())
        {
            constraint.check_hint(letter, hint)?;
        }

        for ((letter, hint), constraint) in guess
            .iter()
            .zip(hints.iter())
            .zip(self.constraints.iter_mut())
        {
            constraint.update_unchecked(letter, hint);

            if hint == Hint::NotExists {
                &mut self.excludes
            } else {
                &mut self.includes
            }
            .insert(letter);

            self.veileds.remove(&letter);
        }

        self.candidates
            .retain(&self.constraints, &self.includes, &self.excludes);

        Ok(())
    }

    pub fn candidates(&self) -> &Candidates {
        &self.candidates
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum UpdateError {
    #[error("contradictory hints: {0}")]
    ContradictoryHints(#[from] position_constraint::CheckHintError),
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
        state.update(&guess, &hints).unwrap();
        assert_eq!(state.includes, letters(b"SR"));
        assert_eq!(state.excludes, letters(b"EIA"));
        assert_eq!(state.veileds, complement(&letters(b"SERIA")));

        let guess = "HYSON".parse().unwrap();
        let hints = "01200".parse().unwrap();
        state.update(&guess, &hints).unwrap();
        assert_eq!(state.includes, letters(b"SRYS"));
        assert_eq!(state.excludes, letters(b"EIAHON"));
        assert_eq!(state.veileds, complement(&letters(b"SERIAHYSON")));
    }

    #[test]
    fn update_contradiction() {
        let mut state = State::default();

        let guess = Word::from_unchecked(0);
        let hints = "20000".parse::<Hints>().unwrap();
        state.update(&guess, &hints).unwrap();

        let hints = "00000".parse::<Hints>().unwrap(); // 0th: 2?0?
        assert!(matches!(
            state.update(&guess, &hints),
            Err(UpdateError::ContradictoryHints(_))
        ));
    }

    #[test]
    fn update_atomicity() {
        let mut state = State::default();

        let guess = Word::from_unchecked(69); // "ABUSE"
        let hints = "00002".parse::<Hints>().unwrap();
        state.update(&guess, &hints).unwrap();

        let guess = Word::from_unchecked(122); // "ACUTE"
        let hints = "01021".parse::<Hints>().unwrap(); // E: 2?1?
        let backup = state.clone();
        let _ = state.update(&guess, &hints).unwrap_err();
        assert_eq!(backup, state);
    }
}
