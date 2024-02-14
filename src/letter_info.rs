use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LetterInfo {
    Not(HashSet<char>),
    Correct(char),
}

impl Default for LetterInfo {
    fn default() -> Self {
        Self::Not(Default::default())
    }
}

impl LetterInfo {
    pub fn as_regex(&self) -> String {
        let not = match self {
            LetterInfo::Not(set) => set,
            LetterInfo::Correct(c) => return (*c).into(),
        };

        if not.is_empty() {
            ".".into()
        } else {
            format!("[^{}]", not.iter().join(""))
        }
    }
}
