mod hint;

use std::{fmt, io::Stdin};

use super::util::get_line;

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
    pub fn read(stdin: &Stdin) -> Self {
        loop {
            eprint!("Hints: ");

            let hints = get_line(stdin);

            match Hints::try_from(hints.as_ref()) {
                Ok(hints) => return hints,
                Err(e) => eprintln!("Failed to read the hints: {e}"),
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Hint> {
        self.0.iter()
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
