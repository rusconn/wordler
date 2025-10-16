use std::str::FromStr;

use crate::{
    dict::WORDS,
    letter::{self, Letter},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guess(Vec<Letter>);

impl FromStr for Guess {
    type Err = ParseError;

    fn from_str(guess: &str) -> Result<Self, Self::Err> {
        if guess.chars().count() != 5 {
            return Err(ParseError::InvalidLength);
        }
        if !WORDS.contains(&guess.to_ascii_uppercase().as_str()) {
            return Err(ParseError::UnknownWord);
        }

        guess
            .chars()
            .map(Letter::try_from)
            .collect::<Result<_, _>>()
            .map(Self)
            .map_err(ParseError::InvalidLetter)
    }
}

impl Guess {
    pub(super) fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.0.iter().copied()
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidLength,
    UnknownWord,
    InvalidLetter(letter::ParseError),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(input, case("audio"), case("STERN"), case("cHuMp"))]
    fn try_from_success(input: &str) {
        assert!(input.parse::<Guess>().is_ok());
    }

    #[rstest(input, case("aaaaa"))]
    fn try_from_failure_word(input: &str) {
        assert_eq!(input.parse::<Guess>().unwrap_err(), ParseError::UnknownWord);
    }

    #[rstest(input, case(""), case("@"), case("will"), case("clippy"))]
    fn try_from_failure_len(input: &str) {
        assert_eq!(
            input.parse::<Guess>().unwrap_err(),
            ParseError::InvalidLength
        );
    }

    #[rstest(input, case("will@"), case("1will"), case("wiあll"), case("wi ll"))]
    fn try_from_failure_letter(input: &str) {
        assert!(input.parse::<Guess>().is_err());
    }
}
