use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hint {
    NotExists,
    WrongSpot,
    CorrectSpot,
}

impl TryFrom<char> for Hint {
    type Error = UnknownHintError;

    fn try_from(hint: char) -> Result<Self, Self::Error> {
        match hint {
            '0' => Ok(Hint::NotExists),
            '1' => Ok(Hint::WrongSpot),
            '2' => Ok(Hint::CorrectSpot),
            _ => Err(UnknownHintError { hint }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownHintError {
    hint: char,
}

impl fmt::Display for UnknownHintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown hint: `{}`", self.hint)
    }
}
