use std::collections::BTreeSet;

use itertools::Itertools;

use crate::letter::Letter;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum LetterInfo {
    #[default]
    Any,
    Not(BTreeSet<Letter>),
    Correct(Letter),
}

impl LetterInfo {
    pub(super) fn not(&mut self, letter: Letter) {
        if let Self::Not(set) = self {
            set.insert(letter);
        } else {
            *self = Self::Not([letter].into());
        }
    }

    pub(super) fn correct(&mut self, letter: Letter) {
        *self = Self::Correct(letter);
    }

    pub(super) fn regex_string(&self) -> String {
        match self {
            Self::Any => ".".into(),
            Self::Not(set) => format!("[^{}]", set.iter().join("")),
            Self::Correct(c) => c.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations() {
        let mut letter_info = LetterInfo::default();
        assert_eq!(letter_info.regex_string(), ".");

        letter_info.not(Letter::from_unchecked(b'A'));
        assert_eq!(letter_info.regex_string(), "[^A]");

        letter_info.correct(Letter::from_unchecked(b'B'));
        assert_eq!(letter_info.regex_string(), "B");

        let mut letter_info = LetterInfo::default();
        letter_info.not(Letter::from_unchecked(b'B'));
        assert_eq!(letter_info.regex_string(), "[^B]");

        letter_info.not(Letter::from_unchecked(b'A'));
        assert_eq!(letter_info.regex_string(), "[^AB]");

        letter_info.correct(Letter::from_unchecked(b'C'));
        assert_eq!(letter_info.regex_string(), "C");
    }
}
