use std::str::FromStr;

use thiserror::Error;

use crate::{dict::WORDS, letter::Letter};

#[derive(Debug, PartialEq, Eq)]
pub struct Guess(Vec<Letter>);

impl FromStr for Guess {
    type Err = ParseError;

    fn from_str(guess: &str) -> Result<Self, Self::Err> {
        if guess.chars().count() != 5 {
            return Err(ParseError::InvalidLength);
        }

        let letters = guess
            .chars()
            .map(Letter::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(ParseError::InvalidLetter)?;

        if !WORDS.contains(&guess.to_ascii_uppercase().as_str()) {
            return Err(ParseError::UnknownWord);
        }

        Ok(Self(letters))
    }
}

impl Guess {
    pub(crate) fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.0.iter().copied()
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
        assert!(input.parse::<Guess>().is_ok());
    }

    #[rstest(input, case(""), case("is"), case("will"), case("clippy"))]
    fn parse_failure_len(input: &str) {
        assert_eq!(input.parse::<Guess>(), Err(ParseError::InvalidLength));
    }

    #[rstest(
        case("will@", '@'),
        case("1will", '1'),
        case("wiあll", 'あ'),
        case("wi ll", ' ')
    )]
    fn parse_failure_letter(#[case] input: &str, #[case] letter: char) {
        let parsed = input.parse::<Guess>();
        assert_eq!(parsed, Err(ParseError::InvalidLetter(letter)));
    }

    #[rstest(input, case("aaaaa"))]
    fn parse_failure_word(input: &str) {
        assert_eq!(input.parse::<Guess>(), Err(ParseError::UnknownWord));
    }
}
