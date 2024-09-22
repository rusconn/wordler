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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self.0))
    }
}

impl Letter {
    pub fn from_unchecked(byte: u8) -> Self {
        Self(byte)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        byte,
        case('a', b'A'),
        case('z', b'Z'),
        case('A', b'A'),
        case('Z', b'Z')
    )]
    fn try_from_success(input: char, byte: u8) {
        assert_eq!(Letter::try_from(input).unwrap(), Letter(byte));
    }

    #[rstest(ch, case('@'), case('1'), case('あ'), case(' '))]
    fn try_from_failure(ch: char) {
        assert_eq!(
            Letter::try_from(ch).unwrap_err().to_string(),
            format!("Non alphabetical letter: `{ch}`"),
        );
    }

    #[rstest(byte, s, case(b'A', "A"), case(b'Z', "Z"))]
    fn fmt(byte: u8, s: &str) {
        assert_eq!(Letter(byte).to_string(), s);
    }
}
