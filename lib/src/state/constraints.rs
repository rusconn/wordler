mod delta;
mod letter;
mod position;

use thiserror::Error;

use crate::{
    Hints, Word,
    dict::{LETTER_KINDS, LETTERS, WORD_LEN},
    letter::Letter,
    letter_set::LetterSet,
};

pub use {
    letter::LetterConstraint,
    position::{CheckError as CheckPositionError, PositionConstraint},
};

use delta::ConstraintDelta;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Constraints {
    positions: [PositionConstraint; WORD_LEN],
    letters: [LetterConstraint; LETTER_KINDS],
    active_letters: LetterSet,
}

impl Constraints {
    pub(crate) fn update(&mut self, guess: Word, hints: &Hints) -> Result<(), UpdateError> {
        let delta = ConstraintDelta::from_feedback(guess, hints);
        let mut next = *self;
        next.merge(delta)?;
        *self = next;
        Ok(())
    }

    fn merge(&mut self, delta: ConstraintDelta) -> Result<(), UpdateError> {
        for (position_constraint, position_delta) in self.positions.iter_mut().zip(delta.positions)
        {
            if let Some(position_delta) = position_delta {
                position_constraint.update(position_delta.letter, position_delta.hint)?;
            }
        }

        for (&letter, letter_delta) in LETTERS.iter().zip(delta.letters) {
            if let Some(letter_delta) = letter_delta {
                let letter_constraint = &mut self.letters[letter.as_index()];
                letter_constraint
                    .update(letter_delta.min_count, letter_delta.max_count)
                    .map_err(|(min, max)| UpdateError::ContradictoryCount { letter, min, max })?;
                if letter_constraint.is_active() {
                    self.active_letters.insert(letter);
                }
            }
        }

        Ok(())
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
            .letters()
            .map(|letter| letter.as_index())
            .all(|index| {
                let count = word.as_letter_counts()[index];
                self.letters[index].is_match(count)
            })
    }

    #[cfg(feature = "recommend")]
    pub(crate) fn is_veiled(&self, letter: Letter) -> bool {
        !self.active_letters.contains(letter)
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum UpdateError {
    #[error(transparent)]
    ContradictoryPosition(#[from] CheckPositionError),

    #[error("contradictory count: letter={letter} min={min}, max={max}")]
    ContradictoryCount { letter: Letter, min: u8, max: u8 },
}
