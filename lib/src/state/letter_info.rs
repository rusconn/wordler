use std::collections::BTreeSet;

use itertools::Itertools;

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
    pub(super) fn update(&mut self, letter: Letter, hint: Hint) {
        if hint == Hint::CorrectSpot {
            self.correct(letter);
        } else {
            self.not(letter);
        }
    }

    fn correct(&mut self, letter: Letter) {
        *self = Self::Correct(letter);
    }

    fn not(&mut self, letter: Letter) {
        if let Self::Not(set) = self {
            set.insert(letter);
        } else {
            *self = Self::Not([letter].into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations() {
        let mut letter_info = LetterInfo::default();
        assert_eq!(letter_info.to_regex_string(), ".");

        letter_info.not(Letter::from_unchecked(b'A'));
        assert_eq!(letter_info.to_regex_string(), "[^A]");

        letter_info.correct(Letter::from_unchecked(b'B'));
        assert_eq!(letter_info.to_regex_string(), "B");

        let mut letter_info = LetterInfo::default();
        letter_info.not(Letter::from_unchecked(b'B'));
        assert_eq!(letter_info.to_regex_string(), "[^B]");

        letter_info.not(Letter::from_unchecked(b'A'));
        assert_eq!(letter_info.to_regex_string(), "[^AB]");

        letter_info.correct(Letter::from_unchecked(b'C'));
        assert_eq!(letter_info.to_regex_string(), "C");
    }
}
