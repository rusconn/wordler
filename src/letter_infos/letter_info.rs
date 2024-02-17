use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::Letter;

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
    Not(FxHashSet<Letter>),
    Correct(Letter),
}

impl Variant {
    fn not(letter: Letter) -> Self {
        Self::Not(FxHashSet::from_iter([letter]))
    }

    fn correct(letter: Letter) -> Self {
        Self::Correct(letter)
    }
}
