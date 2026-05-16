use thiserror::Error;

use crate::{hints::Hint, letter::Letter, letter_set::LetterSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PositionConstraint {
    correct: Option<Letter>,
    not: LetterSet,
}

impl PositionConstraint {
    pub(super) fn update(&mut self, letter: Letter, hint: Hint) -> Result<(), CheckError> {
        if hint == Hint::CorrectSpot {
            self.correct(letter)
        } else {
            self.not(letter)
        }
        .map_err(|letter| CheckError::Contradictory { letter, hint })
    }

    fn correct(&mut self, letter: Letter) -> Result<(), Letter> {
        if let Some(correct) = self.correct
            && correct != letter
        {
            return Err(letter);
        }
        if self.not.contains(letter) {
            return Err(letter);
        }

        self.correct = Some(letter);
        Ok(())
    }

    fn not(&mut self, letter: Letter) -> Result<(), Letter> {
        if let Some(correct) = self.correct
            && correct == letter
        {
            return Err(correct);
        }

        self.not.insert(letter);
        Ok(())
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

        let result = constraint.update(a, Hint::NotExists);
        assert_eq!(result, Ok(()));
        let result = constraint.update(a, Hint::CorrectSpot);
        assert_eq!(
            result,
            Err(CheckError::Contradictory {
                letter: a,
                hint: Hint::CorrectSpot
            })
        );

        let mut constraint = PositionConstraint::default();
        let result = constraint.update(a, Hint::CorrectSpot);
        assert_eq!(result, Ok(()));
        let result = constraint.update(b, Hint::CorrectSpot);
        assert_eq!(
            result,
            Err(CheckError::Contradictory {
                letter: b,
                hint: Hint::CorrectSpot
            })
        );

        let mut constraint = PositionConstraint::default();
        let result = constraint.update(a, Hint::CorrectSpot);
        assert_eq!(result, Ok(()));
        let result = constraint.update(a, Hint::WrongSpot);
        assert_eq!(
            result,
            Err(CheckError::Contradictory {
                letter: a,
                hint: Hint::WrongSpot
            })
        );
    }
}
