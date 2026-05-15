mod delta;
mod letter;
mod position;

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

#[cfg_attr(test, derive(Clone, PartialEq, Eq))]
#[derive(Debug, Default)]
pub(crate) struct Constraints {
    positions: [PositionConstraint; WORD_LEN],
    letters: [LetterConstraint; LETTER_KINDS],
    active_letters: Vec<Letter>,
}

impl Constraints {
    pub(crate) fn update(&mut self, guess: Word, hints: &Hints) -> Result<(), UpdateError> {
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
            if let Some(letter_delta) = delta.letters[letter.as_index()] {
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

        self.rebuild_active_letters();
    }

    fn rebuild_active_letters(&mut self) {
        self.active_letters.clear();
        for &letter in LETTERS.iter() {
            if self.letters[letter.as_index()].is_active() {
                self.active_letters.push(letter);
            }
        }
    }

    pub(crate) fn is_match(&self, word: Word) -> bool {
        self.is_match_positions(word) && self.is_match_counts(word)
    }

    fn is_match_positions(&self, word: Word) -> bool {
        self.positions
            .iter()
            .zip(word.as_letters())
            .all(|(position, letter)| position.is_match(letter))
    }

    fn is_match_counts(&self, word: Word) -> bool {
        self.active_letters
            .iter()
            .map(Letter::as_index)
            .all(|index| {
                let count = word.as_letter_counts()[index];
                self.letters[index].is_match(count)
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
