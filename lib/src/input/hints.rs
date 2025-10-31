pub mod hint;

use std::str::FromStr;

use thiserror::Error;

pub(crate) use self::hint::Hint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hints(Vec<Hint>);

impl FromStr for Hints {
    type Err = ParseError;

    fn from_str(hints: &str) -> Result<Self, Self::Err> {
        if hints.chars().count() != 5 {
            return Err(ParseError::InvalidLength);
        }

        hints
            .chars()
            .map(Hint::try_from)
            .collect::<Result<_, _>>()
            .map(Self)
            .map_err(ParseError::InvalidHint)
    }
}

impl Hints {
    pub(crate) fn iter(&self) -> impl Iterator<Item = Hint> + '_ {
        self.0.iter().copied()
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum ParseError {
    #[error("invalid length")]
    InvalidLength,

    #[error("invalid hint: {0:?}")]
    InvalidHint(char),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        case("00000"),
        case("11111"),
        case("22222"),
        case("01010"),
        case("01201")
    )]
    fn parse_success(input: &str) {
        assert!(input.parse::<Hints>().is_ok());
    }

    #[rstest(input, case(""), case("@"), case("1021"), case("120021"))]
    fn parse_failure_len(input: &str) {
        assert_eq!(input.parse::<Hints>(), Err(ParseError::InvalidLength),);
    }

    #[rstest(
        input,
        case("1021a"),
        case("10b1@"),
        case("10203"),
        case("00あ12"),
        case("10 20")
    )]
    fn parse_failure_hint(input: &str) {
        assert!(input.parse::<Hints>().is_err());
    }
}
