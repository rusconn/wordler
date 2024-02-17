use std::{fmt, io::Stdin};

use crate::letter::{InvalidCharacterError, Letter};

use super::util::get_line;

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
    pub fn read(stdin: &Stdin) -> Self {
        loop {
            eprint!("Guess: ");

            let guess = get_line(stdin);

            match Self::try_from(guess.as_ref()) {
                Ok(guess) => return guess,
                Err(e) => eprintln!("Failed to read the guess: {e}"),
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.0.iter().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidGuessError {
    InvalidLength,
    NonAlphabetical(char),
}

impl From<InvalidCharacterError> for InvalidGuessError {
    fn from(value: InvalidCharacterError) -> Self {
        match value {
            InvalidCharacterError::NonAlphabetical(letter) => Self::NonAlphabetical(letter),
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
