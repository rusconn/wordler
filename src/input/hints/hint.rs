#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hint(pub Variant);

impl TryFrom<char> for Hint {
    type Error = InvalidHintError;

    fn try_from(hint: char) -> Result<Self, Self::Error> {
        match hint {
            '0' => Ok(Self(Variant::NotExists)),
            '1' => Ok(Self(Variant::WrongSpot)),
            '2' => Ok(Self(Variant::CorrectSpot)),
            _ => Err(Self::Error::UnknownHint(hint)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidHintError {
    UnknownHint(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variant {
    NotExists,
    WrongSpot,
    CorrectSpot,
}
