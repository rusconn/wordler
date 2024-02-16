mod letter;

use std::fmt;

use self::letter::{InvalidLetterError, Letter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guess(Vec<Letter>);

impl TryFrom<&str> for Guess {
    type Error = InvalidGuessError;

    fn try_from(guess: &str) -> Result<Self, Self::Error> {
        if guess.chars().count() != 5 {
            return Err(Self::Error::InvalidLength);
        }

        guess
            .chars()
            .map(Letter::try_from)
            .collect::<Result<_, _>>()
            .map(Self)
            .map_err(Self::Error::from)
    }
}

impl Guess {
    pub fn letters(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().map(Letter::as_char)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidGuessError {
    InvalidLength,
    NonAlphabetical(char),
}

impl From<InvalidLetterError> for InvalidGuessError {
    fn from(value: InvalidLetterError) -> Self {
        match value {
            InvalidLetterError::NonAlphabetical(letter) => Self::NonAlphabetical(letter),
        }
    }
}

impl fmt::Display for InvalidGuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Guess must be 5 letters"),
            Self::NonAlphabetical(letter) => write!(f, "Non alphabetical letter: `{letter}`"),
        }
    }
}
