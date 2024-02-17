mod letter_info;

use std::iter;

use itertools::Itertools;

use crate::Letter;

use self::letter_info::LetterInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetterInfos(Vec<LetterInfo>);

impl Default for LetterInfos {
    fn default() -> Self {
        Self(iter::repeat(LetterInfo::default()).take(5).collect())
    }
}

impl LetterInfos {
    pub fn not(&mut self, nth: usize, letter: Letter) {
        self.0[nth].not(letter);
    }

    pub fn correct(&mut self, nth: usize, letter: Letter) {
        self.0[nth].correct(letter);
    }

    pub fn as_regex(&self) -> String {
        self.0.iter().map(LetterInfo::as_regex).join("")
    }
}
