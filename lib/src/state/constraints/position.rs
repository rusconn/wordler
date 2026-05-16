use thiserror::Error;

use crate::{hints::Hint, letter::Letter, letter_set::LetterSet};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PositionConstraint {
    correct: Option<Letter>,
    not: LetterSet,
}

impl PositionConstraint {
    pub(super) fn check(&self, letter: Letter, hint: Hint) -> Result<(), CheckError> {
        if hint == Hint::CorrectSpot {
            self.check_hint_for_correct(letter)
        } else {
            self.check_hint_for_not(letter)
        }
        .map_err(|letter| CheckError::Contradictory { letter, hint })
    }

    fn check_hint_for_correct(&self, letter: Letter) -> Result<(), Letter> {
        if let Some(correct) = self.correct
            && correct != letter
        {
            return Err(letter);
        }
        if self.not.contains(letter) {
            return Err(letter);
        }

        Ok(())
    }

    fn check_hint_for_not(&self, letter: Letter) -> Result<(), Letter> {
        if let Some(correct) = self.correct
            && correct == letter
        {
            Err(correct)
        } else {
            Ok(())
        }
    }

    pub(super) fn update_unchecked(&mut self, letter: Letter, hint: Hint) {
        if hint == Hint::CorrectSpot {
            self.correct_unchecked(letter);
        } else {
            self.not_unchecked(letter);
        }
    }

    fn correct_unchecked(&mut self, letter: Letter) {
        self.correct = Some(letter);
    }

    fn not_unchecked(&mut self, letter: Letter) {
        self.not.insert(letter);
    }

    pub(super) fn is_match(&self, letter: Letter) -> bool {
        if let Some(correct) = self.correct {
            correct == letter
        } else {
            !self.not.contains(letter)
        }
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum CheckError {
    #[error("contradictory hint: (letter: {letter}, hint: {hint})")]
    Contradictory { letter: Letter, hint: Hint },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ckeck_hint() {
        let a = Letter::from_unchecked(b'A');
        let b = Letter::from_unchecked(b'B');

        let mut constraint = PositionConstraint::default();
        constraint.update_unchecked(a, Hint::NotExists);
        let result = constraint.check(a, Hint::NotExists);
        assert_eq!(result, Ok(()));
        let result = constraint.check(a, Hint::CorrectSpot);
        assert_eq!(
            result,
            Err(CheckError::Contradictory {
                letter: a,
                hint: Hint::CorrectSpot
            })
        );

        let mut constraint = PositionConstraint::default();
        constraint.update_unchecked(a, Hint::CorrectSpot);
        let result = constraint.check(a, Hint::CorrectSpot);
        assert_eq!(result, Ok(()));
        let result = constraint.check(b, Hint::CorrectSpot);
        assert_eq!(
            result,
            Err(CheckError::Contradictory {
                letter: b,
                hint: Hint::CorrectSpot
            })
        );

        let mut constraint = PositionConstraint::default();
        constraint.update_unchecked(a, Hint::CorrectSpot);
        let result = constraint.check(a, Hint::CorrectSpot);
        assert_eq!(result, Ok(()));
        let result = constraint.check(a, Hint::WrongSpot);
        assert_eq!(
            result,
            Err(CheckError::Contradictory {
                letter: a,
                hint: Hint::WrongSpot
            })
        );
    }
}
