use std::iter;

use itertools::Itertools;

use crate::letter_info::LetterInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetterInfos {
    infos: Vec<LetterInfo>,
}

impl LetterInfos {
    pub fn new(n: usize) -> Self {
        Self {
            infos: iter::repeat(LetterInfo::default()).take(n).collect(),
        }
    }

    pub fn not(&mut self, nth: usize, letter: char) {
        let info = match self.infos.get_mut(nth) {
            Some(info) => info,
            None => panic!("Invalid index: {nth}"),
        };

        if let LetterInfo::Not(set) = info {
            set.insert(letter);
        }
    }

    pub fn correct(&mut self, nth: usize, letter: char) {
        let info = match self.infos.get_mut(nth) {
            Some(info) => info,
            None => panic!("Invalid index: {nth}"),
        };

        *info = LetterInfo::Correct(letter);
    }

    pub fn as_regex(&self) -> String {
        self.infos.iter().map(LetterInfo::as_regex).join("")
    }
}
