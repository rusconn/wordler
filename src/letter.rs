use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Letter(u8);

impl TryFrom<char> for Letter {
    type Error = ParseError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if !ch.is_ascii_alphabetic() {
            return Err(ParseError::NonAlphabeticalLetter(ch));
        }

        Ok(Self(ch.to_ascii_uppercase() as u8))
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self.0))
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NonAlphabeticalLetter(char),
}

impl Letter {
    pub(crate) fn from_unchecked(byte: u8) -> Self {
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
            Letter::try_from(ch).unwrap_err(),
            ParseError::NonAlphabeticalLetter(ch)
        );
    }

    #[rstest(byte, s, case(b'A', "A"), case(b'Z', "Z"))]
    fn fmt(byte: u8, s: &str) {
        assert_eq!(Letter(byte).to_string(), s);
    }
}
