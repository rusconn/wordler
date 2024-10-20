use std::{collections::BTreeSet, iter};

use itertools::Itertools;

use crate::letter::Letter;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LetterInfo(Variant);

impl LetterInfo {
    pub(super) fn not(&mut self, letter: Letter) {
        if let Variant::Not(set) = &mut self.0 {
            set.insert(letter);
        } else {
            self.0 = Variant::not(letter);
        }
    }

    pub(super) fn correct(&mut self, letter: Letter) {
        self.0 = Variant::correct(letter);
    }

    pub(super) fn regex_string(&self) -> String {
        match &self.0 {
            Variant::Any => ".".into(),
            Variant::Not(set) => format!("[^{}]", set.iter().join("")),
            Variant::Correct(c) => c.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum Variant {
    #[default]
    Any,
    Not(BTreeSet<Letter>),
    Correct(Letter),
}

impl Variant {
    fn not(letter: Letter) -> Self {
        Self::Not(iter::once(letter).collect())
    }

    fn correct(letter: Letter) -> Self {
        Self::Correct(letter)
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
