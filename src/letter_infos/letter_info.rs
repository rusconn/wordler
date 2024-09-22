use itertools::Itertools;

use crate::{letter::Letter, letter_set::NotLetters};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetterInfo(Variant);

impl Default for LetterInfo {
    fn default() -> Self {
        Self(Variant::Any)
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Variant {
    Any,
    Not(NotLetters),
    Correct(Letter),
}

impl Variant {
    fn not(letter: Letter) -> Self {
        Self::Not(NotLetters::new(letter))
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
