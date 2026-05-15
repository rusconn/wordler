mod delta;
mod letter;
mod position;

use std::iter;

use itertools::Itertools;
use regex::Regex;
use thiserror::Error;

use crate::{
    Hints, Word,
    dict::{LETTER_KINDS, LETTERS, WORD_LEN},
    letter::Letter,
};

pub use {
    letter::LetterConstraint,
    position::{CheckError as CheckPositionError, PositionConstraint},
};

use delta::ConstraintDelta;

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
        let delta = ConstraintDelta::from_feedback(guess, hints);
        self.check(&delta)?;
        self.merge_unchecked(delta);
        Ok(())
    }

    fn check(&self, delta: &ConstraintDelta) -> Result<(), UpdateError> {
        for (position_constraint, position_delta) in
            self.positions.iter().zip(delta.positions.iter())
        {
            if let Some(position_delta) = position_delta {
                position_constraint.check(position_delta.letter, position_delta.hint)?;
            }
        }

        for (&letter, letter_constraint) in LETTERS.iter().zip(self.letters.iter()) {
            if let Some(letter_delta) = &delta.letters[letter.as_index()] {
                letter_constraint
                    .check(letter_delta.min_count, letter_delta.max_count)
                    .map_err(|(min, max)| UpdateError::ContradictoryCount { letter, min, max })?;
            }
        }

        Ok(())
    }

    fn merge_unchecked(&mut self, delta: ConstraintDelta) {
        for (position_constraint, position_delta) in self.positions.iter_mut().zip(delta.positions)
        {
            if let Some(position_delta) = position_delta {
                position_constraint.update_unchecked(position_delta.letter, position_delta.hint);
            }
        }

        for (letter, letter_delta) in LETTERS.iter().zip(delta.letters) {
            if let Some(letter_delta) = letter_delta {
                let letter_constraint = &mut self.letters[letter.as_index()];
                letter_constraint.update_unchecked(letter_delta.min_count, letter_delta.max_count);
            }
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
