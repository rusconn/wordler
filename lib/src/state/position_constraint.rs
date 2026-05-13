use std::collections::BTreeSet;

use itertools::Itertools;
use thiserror::Error;

use crate::{hints::Hint, letter::Letter};

use super::to_regex_string::ToRegexString;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct PositionConstraint {
    correct: Option<Letter>,
    not: BTreeSet<Letter>,
}

impl ToRegexString for PositionConstraint {
    fn to_regex_string(&self) -> String {
        if let Some(letter) = self.correct {
            return letter.to_string();
        }
        if self.not.is_empty() {
            ".".into()
        } else {
            format!("[^{}]", self.not.iter().join(""))
        }
    }
}

impl ToRegexString for [PositionConstraint] {
    fn to_regex_string(&self) -> String {
        self.iter()
            .map(PositionConstraint::to_regex_string)
            .join("")
    }
}

impl PositionConstraint {
    pub(super) fn check_hint(&self, letter: Letter, hint: Hint) -> Result<(), CheckHintError> {
        if hint == Hint::CorrectSpot {
            self.check_hint_for_correct(letter)
        } else {
            self.check_hint_for_not(letter)
        }
        .map_err(|letter| CheckHintError::Contradictory { letter, hint })
    }

    fn check_hint_for_correct(&self, letter: Letter) -> Result<(), Letter> {
        if let Some(correct) = self.correct
            && correct != letter
        {
            return Err(letter);
        }
        if self.not.contains(&letter) {
            return Err(letter);
        };

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
}

#[derive(Debug, PartialEq, Error)]
pub enum CheckHintError {
    #[error("contradictory hint: (letter: {letter}, hint: {hint})")]
    Contradictory { letter: Letter, hint: Hint },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations() {
        let mut constraint = PositionConstraint::default();
        assert_eq!(constraint.to_regex_string(), ".");

        constraint.not_unchecked(Letter::from_unchecked(b'A'));
        assert_eq!(constraint.to_regex_string(), "[^A]");

        constraint.correct_unchecked(Letter::from_unchecked(b'B'));
        assert_eq!(constraint.to_regex_string(), "B");

        let mut constraint = PositionConstraint::default();
        constraint.not_unchecked(Letter::from_unchecked(b'B'));
        assert_eq!(constraint.to_regex_string(), "[^B]");

        constraint.not_unchecked(Letter::from_unchecked(b'A'));
        assert_eq!(constraint.to_regex_string(), "[^AB]");

        constraint.correct_unchecked(Letter::from_unchecked(b'C'));
        assert_eq!(constraint.to_regex_string(), "C");
    }

    #[test]
    fn ckeck_hint() {
        let a = Letter::from_unchecked(b'A');
        let b = Letter::from_unchecked(b'B');

        let mut constraint = PositionConstraint::default();
        constraint.update_unchecked(a, Hint::NotExists);
        let result = constraint.check_hint(a, Hint::NotExists);
        assert_eq!(result, Ok(()));
        let result = constraint.check_hint(a, Hint::CorrectSpot);
        assert_eq!(
            result,
            Err(CheckHintError::Contradictory {
                letter: a,
                hint: Hint::CorrectSpot
            })
        );

        let mut constraint = PositionConstraint::default();
        constraint.update_unchecked(a, Hint::CorrectSpot);
        let result = constraint.check_hint(a, Hint::CorrectSpot);
        assert_eq!(result, Ok(()));
        let result = constraint.check_hint(b, Hint::CorrectSpot);
        assert_eq!(
            result,
            Err(CheckHintError::Contradictory {
                letter: b,
                hint: Hint::CorrectSpot
            })
        );

        let mut constraint = PositionConstraint::default();
        constraint.update_unchecked(a, Hint::CorrectSpot);
        let result = constraint.check_hint(a, Hint::CorrectSpot);
        assert_eq!(result, Ok(()));
        let result = constraint.check_hint(a, Hint::WrongSpot);
        assert_eq!(
            result,
            Err(CheckHintError::Contradictory {
                letter: a,
                hint: Hint::WrongSpot
            })
        );
    }
}
