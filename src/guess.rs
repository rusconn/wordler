use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guess(String);

impl TryFrom<&str> for Guess {
    type Error = InvalidGuessError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().count() != 5 {
            return Err(Self::Error::InvalidLength);
        }
        if !value.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(Self::Error::NonAlphabetical);
        }

        Ok(Self(value.into()))
    }
}

impl Guess {
    pub fn letters(&self) -> impl Iterator<Item = char> + '_ {
        self.0.chars()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidGuessError {
    InvalidLength,
    NonAlphabetical,
}

impl fmt::Display for InvalidGuessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Guess must be 5 letters"),
            Self::NonAlphabetical => write!(f, "All letters must be ascii alphabetic"),
        }
    }
}
