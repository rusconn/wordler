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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_success() {
        assert_eq!(Letter::try_from('a').unwrap(), Letter(b'A'));
        assert_eq!(Letter::try_from('z').unwrap(), Letter(b'Z'));
        assert_eq!(Letter::try_from('A').unwrap(), Letter(b'A'));
        assert_eq!(Letter::try_from('Z').unwrap(), Letter(b'Z'));
    }

    #[test]
    fn try_from_failure() {
        assert_eq!(
            Letter::try_from('@').unwrap_err().to_string(),
            "Non alphabetical letter: `@`"
        );
        assert_eq!(
            Letter::try_from('1').unwrap_err().to_string(),
            "Non alphabetical letter: `1`"
        );
        assert_eq!(
            Letter::try_from('あ').unwrap_err().to_string(),
            "Non alphabetical letter: `あ`"
        );
        assert_eq!(
            Letter::try_from(' ').unwrap_err().to_string(),
            "Non alphabetical letter: ` `"
        );
    }

    #[test]
    fn fmt() {
        assert_eq!(Letter(b'A').to_string(), "A");
        assert_eq!(Letter(b'Z').to_string(), "Z");
    }
}
