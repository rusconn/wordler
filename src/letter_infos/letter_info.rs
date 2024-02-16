use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetterInfo(Variant);

impl Default for LetterInfo {
    fn default() -> Self {
        Self(Variant::Any)
    }
}

impl LetterInfo {
    pub fn not(&mut self, letter: char) {
        if let Variant::Not(set) = &mut self.0 {
            set.insert(letter);
        } else {
            self.0 = Variant::not(letter);
        }
    }

    pub fn correct(&mut self, letter: char) {
        self.0 = Variant::correct(letter);
    }

    pub fn as_regex(&self) -> String {
        match &self.0 {
            Variant::Any => ".".into(),
            Variant::Not(set) => format!("[^{}]", set.iter().join("")),
            Variant::Correct(c) => (*c).into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Variant {
    Any,
    Not(HashSet<char>),
    Correct(char),
}

impl Variant {
    fn not(letter: char) -> Self {
        Self::Not(HashSet::from([letter]))
    }

    fn correct(letter: char) -> Self {
        Self::Correct(letter)
    }
}
