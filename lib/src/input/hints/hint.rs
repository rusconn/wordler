use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Hint {
    NotExists,
    WrongSpot,
    CorrectSpot,
}

impl TryFrom<char> for Hint {
    type Error = ParseError;

    fn try_from(hint: char) -> Result<Self, Self::Error> {
        match hint {
            '0' => Ok(Self::NotExists),
            '1' => Ok(Self::WrongSpot),
            '2' => Ok(Self::CorrectSpot),
            _ => Err(ParseError::InvalidHint(hint)),
        }
    }
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum ParseError {
    #[error("invalid hint: {0:?}")]
    InvalidHint(char),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        hint,
        case('0', Hint::NotExists),
        case('1', Hint::WrongSpot),
        case('2', Hint::CorrectSpot)
    )]
    fn try_from_success(input: char, hint: Hint) {
        assert_eq!(Hint::try_from(input).unwrap(), hint);
    }

    #[rstest(input, case('@'), case('3'), case('あ'), case(' '))]
    fn try_from_failure(input: char) {
        assert_eq!(
            Hint::try_from(input).unwrap_err(),
            ParseError::InvalidHint(input)
        );
    }
}
