use std::iter;

use itertools::Itertools;

use crate::char_info::CharInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharInfos {
    infos: Vec<CharInfo>,
}

impl CharInfos {
    pub fn new(n: usize) -> Self {
        Self {
            infos: iter::repeat(CharInfo::default()).take(n).collect(),
        }
    }

    pub fn not(&mut self, index: usize, ch: char) {
        let info = match self.infos.get_mut(index) {
            Some(info) => info,
            None => panic!("Invalid index: {index}"),
        };

        if let CharInfo::Not(set) = info {
            set.insert(ch);
        }
    }

    pub fn correct(&mut self, index: usize, ch: char) {
        let info = match self.infos.get_mut(index) {
            Some(info) => info,
            None => panic!("Invalid index: {index}"),
        };

        *info = CharInfo::Correct(ch);
    }

    pub fn as_regex(&self) -> String {
        self.infos.iter().map(CharInfo::as_regex).join("")
    }
}
