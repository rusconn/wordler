mod position;

use std::iter;

use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashSet;
use thiserror::Error;

use crate::{Hints, Word, hints::Hint, letter::Letter};

pub use position::{CheckHintError as CheckPositionHintError, PositionConstraint};

#[cfg_attr(test, derive(Clone))]
#[derive(Debug)]
pub(crate) struct Constraints {
    positions: Vec<PositionConstraint>,
    includes: FxHashSet<Letter>,
    excludes: FxHashSet<Letter>,
    regex_cache: Regex,
}

#[cfg(test)]
impl PartialEq for Constraints {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions
            && self.includes == other.includes
            && self.excludes == other.excludes
            && self.regex_cache.as_str() == other.regex_cache.as_str()
    }
}

#[cfg(test)]
impl Eq for Constraints {}

impl Default for Constraints {
    fn default() -> Self {
        Self {
            positions: iter::repeat_n(PositionConstraint::default(), 5).collect(),
            includes: Default::default(),
            excludes: Default::default(),
            regex_cache: Regex::new("").unwrap(),
        }
    }
}

impl Constraints {
    pub(crate) fn update(&mut self, guess: &Word, hints: &Hints) -> Result<(), UpdateError> {
        for ((letter, hint), position_constraint) in guess
            .iter() //
            .zip(hints.iter())
            .zip(self.positions.iter())
        {
            position_constraint.check_hint(letter, hint)?;
        }

        for ((letter, hint), position_constraint) in guess
            .iter()
            .zip(hints.iter())
            .zip(self.positions.iter_mut())
        {
            position_constraint.update_unchecked(letter, hint);

            if hint == Hint::NotExists {
                &mut self.excludes
            } else {
                &mut self.includes
            }
            .insert(letter);
        }

        self.regex_cache = Regex::new(
            &self
                .positions
                .iter()
                .map(PositionConstraint::to_regex_string)
                .join(""),
        )
        .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"));

        Ok(())
    }

    pub(crate) fn is_match(&self, word: &Word) -> bool {
        self.regex_cache.is_match(word.as_str())
            && self.includes.is_subset(word.as_letter_set())
            && self.excludes.is_disjoint(word.as_letter_set())
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("contradictory hints: {0}")]
    ContradictoryHints(#[from] CheckPositionHintError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn letters(bytes: &[u8]) -> FxHashSet<Letter> {
        bytes.iter().copied().map(Letter::from_unchecked).collect()
    }

    #[test]
    fn update() {
        let mut constraints = Constraints::default();
        assert_eq!(constraints.includes, letters(b""));
        assert_eq!(constraints.excludes, letters(b""));

        let guess = "SERIA".parse().unwrap();
        let hints = "10100".parse().unwrap();
        constraints.update(&guess, &hints).unwrap();
        assert_eq!(constraints.includes, letters(b"SR"));
        assert_eq!(constraints.excludes, letters(b"EIA"));

        let guess = "HYSON".parse().unwrap();
        let hints = "01200".parse().unwrap();
        constraints.update(&guess, &hints).unwrap();
        assert_eq!(constraints.includes, letters(b"SRYS"));
        assert_eq!(constraints.excludes, letters(b"EIAHON"));
    }
}
