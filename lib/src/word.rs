use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    dict::{self, LETTER_KINDS, WORD_LEN},
    letter::Letter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word(usize);

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Word {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.chars().count() != WORD_LEN {
            return Err(ParseError::InvalidLength);
        }

        for c in input.chars() {
            Letter::try_from(c).map_err(ParseError::InvalidLetter)?;
        }

        dict::WORD_STRINGS
            .binary_search(&input.to_ascii_uppercase().as_str())
            .map(Self)
            .map_err(|_| ParseError::UnknownWord)
    }
}

impl Word {
    pub(crate) fn from_unchecked(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn as_index(&self) -> usize {
        self.0
    }

    pub(crate) fn as_str(&self) -> &'static str {
        dict::WORD_STRINGS[self.as_index()]
    }

    pub(crate) fn as_letters(&self) -> impl Iterator<Item = Letter> {
        self.as_str().bytes().map(Letter::from_unchecked)
    }

    pub(crate) fn as_letter_counts(&self) -> &'static [u8; LETTER_KINDS] {
        &dict::WORD_LETTER_COUNTS[self.as_index()]
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum ParseError {
    #[error("invalid length")]
    InvalidLength,

    #[error("unknown word")]
    UnknownWord,

    #[error("invalid letter")]
    InvalidLetter(char),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(input, case("audio"), case("STERN"), case("cHuMp"))]
    fn parse_success(input: &str) {
        assert!(input.parse::<Word>().is_ok());
    }

    #[rstest(input, case(""), case("is"), case("will"), case("clippy"))]
    fn parse_failure_len(input: &str) {
        assert_eq!(input.parse::<Word>(), Err(ParseError::InvalidLength));
    }

    #[rstest(
        case("will@", '@'),
        case("1will", '1'),
        case("wiあll", 'あ'),
        case("wi ll", ' ')
    )]
    fn parse_failure_letter(#[case] input: &str, #[case] letter: char) {
        let parsed = input.parse::<Word>();
        assert_eq!(parsed, Err(ParseError::InvalidLetter(letter)));
    }

    #[rstest(input, case("aaaaa"))]
    fn parse_failure_word(input: &str) {
        assert_eq!(input.parse::<Word>(), Err(ParseError::UnknownWord));
    }
}
