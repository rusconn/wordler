use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharInfo {
    Not(HashSet<char>),
    Correct(char),
}

impl Default for CharInfo {
    fn default() -> Self {
        Self::Not(Default::default())
    }
}

impl CharInfo {
    pub fn as_regex(&self) -> String {
        let not = match self {
            CharInfo::Not(set) => set,
            CharInfo::Correct(c) => return (*c).into(),
        };

        if not.is_empty() {
            ".".into()
        } else {
            format!("[^{}]", not.iter().join(""))
        }
    }
}
