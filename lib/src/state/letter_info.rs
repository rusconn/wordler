use std::collections::BTreeSet;

use itertools::Itertools;
use thiserror::Error;

use crate::{hints::Hint, letter::Letter};

use super::to_regex_string::ToRegexString;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum LetterInfo {
    #[default]
    Any,
    Not(BTreeSet<Letter>),
    Correct(Letter),
}

impl ToRegexString for LetterInfo {
    fn to_regex_string(&self) -> String {
        match self {
            Self::Any => ".".into(),
            Self::Not(set) => format!("[^{}]", set.iter().join("")),
            Self::Correct(c) => c.to_string(),
        }
    }
}

impl ToRegexString for [LetterInfo] {
    fn to_regex_string(&self) -> String {
        self.iter().map(LetterInfo::to_regex_string).join("")
    }
}

impl LetterInfo {
    pub(super) fn check_hint(&self, letter: Letter, hint: Hint) -> Result<(), CheckHintError> {
        if hint == Hint::CorrectSpot {
            self.check_hint_for_correct(letter)
        } else {
            self.check_hint_for_not(letter)
        }
        .map_err(|letter| CheckHintError::Contradictory { letter, hint })
    }

    fn check_hint_for_correct(&self, letter: Letter) -> Result<(), Letter> {
        if let LetterInfo::Not(set) = self
            && set.contains(&letter)
        {
            return Err(letter);
        };
        if let LetterInfo::Correct(l) = self
            && *l != letter
        {
            return Err(letter);
        }

        Ok(())
    }

    fn check_hint_for_not(&self, letter: Letter) -> Result<(), Letter> {
        if let Self::Correct(l) = self {
            if *l == letter { Err(letter) } else { Ok(()) }
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
        *self = Self::Correct(letter);
    }

    fn not_unchecked(&mut self, letter: Letter) {
        if let Self::Not(set) = self {
            set.insert(letter);
        } else {
            *self = Self::Not([letter].into());
        }
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
        let mut letter_info = LetterInfo::default();
        assert_eq!(letter_info.to_regex_string(), ".");

        letter_info.not_unchecked(Letter::from_unchecked(b'A'));
        assert_eq!(letter_info.to_regex_string(), "[^A]");

        letter_info.correct_unchecked(Letter::from_unchecked(b'B'));
        assert_eq!(letter_info.to_regex_string(), "B");

        let mut letter_info = LetterInfo::default();
        letter_info.not_unchecked(Letter::from_unchecked(b'B'));
        assert_eq!(letter_info.to_regex_string(), "[^B]");

        letter_info.not_unchecked(Letter::from_unchecked(b'A'));
        assert_eq!(letter_info.to_regex_string(), "[^AB]");

        letter_info.correct_unchecked(Letter::from_unchecked(b'C'));
        assert_eq!(letter_info.to_regex_string(), "C");
    }

    #[test]
    fn ckeck_hint() {
        let a = Letter::from_unchecked(b'A');
        let b = Letter::from_unchecked(b'B');

        let mut letter_info = LetterInfo::default();
        letter_info.update_unchecked(a, Hint::NotExists);
        let result = letter_info.check_hint(a, Hint::NotExists);
        assert_eq!(result, Ok(()));
        let result = letter_info.check_hint(a, Hint::CorrectSpot);
        assert_eq!(
            result,
            Err(CheckHintError::Contradictory {
                letter: a,
                hint: Hint::CorrectSpot
            })
        );

        let mut letter_info = LetterInfo::default();
        letter_info.update_unchecked(a, Hint::CorrectSpot);
        let result = letter_info.check_hint(a, Hint::CorrectSpot);
        assert_eq!(result, Ok(()));
        let result = letter_info.check_hint(b, Hint::CorrectSpot);
        assert_eq!(
            result,
            Err(CheckHintError::Contradictory {
                letter: b,
                hint: Hint::CorrectSpot
            })
        );

        let mut letter_info = LetterInfo::default();
        letter_info.update_unchecked(a, Hint::CorrectSpot);
        let result = letter_info.check_hint(a, Hint::CorrectSpot);
        assert_eq!(result, Ok(()));
        let result = letter_info.check_hint(a, Hint::WrongSpot);
        assert_eq!(
            result,
            Err(CheckHintError::Contradictory {
                letter: a,
                hint: Hint::WrongSpot
            })
        );
    }
}
