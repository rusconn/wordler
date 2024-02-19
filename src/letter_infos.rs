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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations() {
        let mut letter_infos = LetterInfos::default();
        assert_eq!(letter_infos.as_regex(), ".....");

        letter_infos.not(1, Letter::unsafe_from(b'A'));
        assert_eq!(letter_infos.as_regex(), ".[^A]...");

        letter_infos.not(4, Letter::unsafe_from(b'B'));
        assert_eq!(letter_infos.as_regex(), ".[^A]..[^B]");

        letter_infos.not(1, Letter::unsafe_from(b'C'));
        assert_eq!(letter_infos.as_regex(), ".[^AC]..[^B]");

        letter_infos.correct(2, Letter::unsafe_from(b'D'));
        assert_eq!(letter_infos.as_regex(), ".[^AC]D.[^B]");

        letter_infos.correct(1, Letter::unsafe_from(b'E'));
        assert_eq!(letter_infos.as_regex(), ".ED.[^B]");
    }
}
