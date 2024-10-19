use std::iter;

use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::letter::Letter;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LetterInfo(Variant);

impl LetterInfo {
    pub fn not(&mut self, letter: Letter) {
        if let Variant::Not(set) = &mut self.0 {
            set.insert(letter);
        } else {
            self.0 = Variant::not(letter);
        }
    }

    pub fn correct(&mut self, letter: Letter) {
        self.0 = Variant::correct(letter);
    }

    pub fn as_regex(&self) -> String {
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
    Not(FxHashSet<Letter>),
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
        assert_eq!(letter_info.as_regex(), ".");

        letter_info.not(Letter::from_unchecked(b'A'));
        assert_eq!(letter_info.as_regex(), "[^A]");

        letter_info.correct(Letter::from_unchecked(b'B'));
        assert_eq!(letter_info.as_regex(), "B");

        let mut letter_info = LetterInfo::default();
        letter_info.not(Letter::from_unchecked(b'A'));
        assert_eq!(letter_info.as_regex(), "[^A]");

        letter_info.not(Letter::from_unchecked(b'B'));
        assert!(["[^AB]", "[^BA]"].contains(&letter_info.as_regex().as_str()));

        letter_info.correct(Letter::from_unchecked(b'C'));
        assert_eq!(letter_info.as_regex(), "C");
    }
}
