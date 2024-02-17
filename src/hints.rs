mod hint;

use std::{collections::HashSet, fmt};

use crate::{Guess, LetterInfos};

use self::hint::{Hint, InvalidHintError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hints(Vec<Hint>);

impl TryFrom<&str> for Hints {
    type Error = InvalidHintsError;

    fn try_from(hints: &str) -> Result<Self, Self::Error> {
        if hints.chars().count() != 5 {
            return Err(Self::Error::InvalidLength);
        }

        hints
            .chars()
            .map(Hint::try_from)
            .collect::<Result<_, _>>()
            .map(Self)
            .map_err(Self::Error::from)
    }
}

impl Hints {
    pub fn apply(
        &self,
        guess: &Guess,
        letter_infos: &mut LetterInfos,
        contains: &mut HashSet<char>,
        not_contains: &mut HashSet<char>,
        unuseds: &mut HashSet<char>,
    ) {
        for (nth, (letter, hint)) in guess.letters().zip(&self.0).enumerate() {
            hint.apply(nth, letter, letter_infos, contains, not_contains, unuseds);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidHintsError {
    InvalidLength,
    UnknownHint(char),
}

impl From<InvalidHintError> for InvalidHintsError {
    fn from(value: InvalidHintError) -> Self {
        match value {
            InvalidHintError::UnknownHint(hint) => Self::UnknownHint(hint),
        }
    }
}

impl fmt::Display for InvalidHintsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Hints must be 5 letters"),
            Self::UnknownHint(hint) => write!(f, "Unknown hint: `{hint}`"),
        }
    }
}
