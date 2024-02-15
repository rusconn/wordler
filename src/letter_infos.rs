mod letter_info;

use std::iter;

use itertools::Itertools;

use self::letter_info::LetterInfo;

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
        self.infos[nth].not(letter);
    }

    pub fn correct(&mut self, nth: usize, letter: char) {
        self.infos[nth].correct(letter);
    }

    pub fn as_regex(&self) -> String {
        self.infos.iter().map(LetterInfo::as_regex).join("")
    }
}
