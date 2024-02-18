use std::{error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Letter(u8);

impl TryFrom<char> for Letter {
    type Error = Box<dyn error::Error>;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if !ch.is_ascii_alphabetic() {
            return Err(NonAlphabeticalCharacterError(ch).into());
        }

        Ok(Self(ch.to_ascii_uppercase() as u8))
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl Letter {
    /// Make sure the byte is valid
    pub fn unsafe_from(byte: u8) -> Self {
        Self(byte)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NonAlphabeticalCharacterError(char);

impl fmt::Display for NonAlphabeticalCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Non alphabetical letter: `{}`", self.0)
    }
}

impl error::Error for NonAlphabeticalCharacterError {}
