use std::fmt;

use regex::Regex;
use rustc_hash::FxHashSet;

use crate::{letter::Letter, letter_infos::LetterInfos};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    word: &'a str,
    letters: Letters,
}

pub type Letters = FxHashSet<Letter>;

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> Word<'a> {
    pub fn from_unchecked(str: &'a str) -> Self {
        Self {
            word: str,
            letters: str.bytes().map(Letter::from_unchecked).collect(),
        }
    }

    pub fn unique_letters(&self) -> impl Iterator<Item = Letter> + '_ {
        self.letters.iter().copied()
    }

    pub fn is_match(&self, letter_infos: &LetterInfos, regex: &Regex) -> bool {
        letter_infos.is_match(&self.letters) && regex.is_match(self.word)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        output,
        case("AUDIO", "AUDIO"),
        case("HIPPO", "HIPPO"),
        case("AAAAA", "AAAAA")
    )]
    fn fmt(input: &str, output: &str) {
        assert_eq!(Word::from_unchecked(input).to_string(), output);
    }

    #[rstest(input, output, case("AUDIO", 5), case("HIPPO", 4), case("AAAAA", 1))]
    fn unique_letters(input: &str, output: usize) {
        assert_eq!(Word::from_unchecked(input).unique_letters().count(), output);
    }
}
