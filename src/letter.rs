use std::fmt;

use anyhow::{ensure, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Letter(u8);

impl TryFrom<char> for Letter {
    type Error = anyhow::Error;

    fn try_from(ch: char) -> Result<Self> {
        ensure!(ch.is_ascii_alphabetic(), "Non alphabetical letter: `{ch}`");

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
