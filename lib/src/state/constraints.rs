mod letter;
mod position;

use std::iter;

use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;
use thiserror::Error;

use crate::{
    Hints, Word,
    dict::{LETTER_KINDS, LETTERS, WORD_LEN},
    hints::Hint,
    letter::Letter,
};

pub use {
    letter::LetterConstraint,
    position::{CheckError as CheckPositionError, PositionConstraint},
};

#[cfg_attr(test, derive(Clone))]
#[derive(Debug)]
pub(crate) struct Constraints {
    positions: [PositionConstraint; WORD_LEN],
    letters: [LetterConstraint; LETTER_KINDS],
    regex_cache: Regex,
}

#[cfg(test)]
impl PartialEq for Constraints {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions
            && self.letters == other.letters
            && self.regex_cache.as_str() == other.regex_cache.as_str()
    }
}

#[cfg(test)]
impl Eq for Constraints {}

impl Default for Constraints {
    fn default() -> Self {
        Self {
            positions: iter::repeat_n(Default::default(), WORD_LEN)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            letters: iter::repeat_n(Default::default(), LETTER_KINDS)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            regex_cache: Regex::new("").unwrap(),
        }
    }
}

impl Constraints {
    pub(crate) fn update(&mut self, guess: &Word, hints: &Hints) -> Result<(), UpdateError> {
        self.check(guess, hints)?;
        self.update_unchecked(guess, hints);
        Ok(())
    }

    fn check(&self, guess: &Word, hints: &Hints) -> Result<(), UpdateError> {
        for ((letter, hint), position_constraint) in guess
            .iter() //
            .zip(hints.iter())
            .zip(self.positions.iter())
        {
            position_constraint.check(letter, hint)?;
        }

        for (letter, (min, max)) in Self::feedback_letter_counts(guess, hints) {
            let letter_constraint = &self.letters[letter.as_index()];
            letter_constraint
                .check(min, max)
                .map_err(|(min, max)| UpdateError::ContradictoryCount { letter, min, max })?;
        }

        Ok(())
    }

    fn feedback_letter_counts(guess: &Word, hints: &Hints) -> Vec<(Letter, (u8, Option<u8>))> {
        let mut map = FxHashMap::<Letter, (u8, u8)>::default();

        for (letter, hint) in guess.iter().zip(hints.iter()) {
            let (green_or_yellow, gray) = map.entry(letter).or_insert((0, 0));
            if hint == Hint::NotExists {
                *gray += 1;
            } else {
                *green_or_yellow += 1;
            }
        }

        map.into_iter()
            .map(|(letter, (green_or_yellow, gray))| {
                if gray == 0 {
                    (letter, (green_or_yellow, None))
                } else {
                    (letter, (green_or_yellow, Some(green_or_yellow)))
                }
            })
            .collect()
    }

    fn update_unchecked(&mut self, guess: &Word, hints: &Hints) {
        for ((letter, hint), position_constraint) in guess
            .iter()
            .zip(hints.iter())
            .zip(self.positions.iter_mut())
        {
            position_constraint.update_unchecked(letter, hint);
        }

        for (letter, (min, max)) in Self::feedback_letter_counts(guess, hints) {
            let letter_constraint = &mut self.letters[letter.as_index()];
            letter_constraint.update_unchecked(min, max);
        }

        self.regex_cache = Regex::new(
            &self
                .positions
                .iter()
                .map(PositionConstraint::to_regex_string)
                .join(""),
        )
        .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"));
    }

    pub(crate) fn is_match(&self, word: &Word) -> bool {
        self.is_match_positions(word) && self.is_match_counts(word)
    }

    fn is_match_positions(&self, word: &Word) -> bool {
        self.regex_cache.is_match(word.as_str())
    }

    fn is_match_counts(&self, word: &Word) -> bool {
        LETTERS
            .iter()
            .zip(&self.letters)
            .all(|(letter, letter_constraint)| {
                let count = word
                    .as_letter_counts()
                    .iter()
                    .find(|(l, _)| l == letter)
                    .map(|&(_, count)| count)
                    .unwrap_or(0);
                letter_constraint.is_match(count)
            })
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum UpdateError {
    #[error(transparent)]
    ContradictoryPosition(#[from] CheckPositionError),

    #[error("contradictory count: letter={letter} min={min}, max={max}")]
    ContradictoryCount { letter: Letter, min: u8, max: u8 },
}
