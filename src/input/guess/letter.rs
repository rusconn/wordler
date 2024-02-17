#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Letter(char);

impl TryFrom<char> for Letter {
    type Error = InvalidLetterError;

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        if !letter.is_ascii_alphabetic() {
            return Err(Self::Error::NonAlphabetical(letter));
        }

        Ok(Self(letter.to_ascii_uppercase()))
    }
}

impl Letter {
    pub fn as_char(&self) -> char {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidLetterError {
    NonAlphabetical(char),
}
