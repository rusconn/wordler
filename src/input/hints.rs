pub mod hint;

pub(super) use self::hint::Hint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hints(Vec<Hint>);

impl TryFrom<&str> for Hints {
    type Error = ParseError;

    fn try_from(hints: &str) -> Result<Self, Self::Error> {
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
    pub(super) fn iter(&self) -> impl Iterator<Item = Hint> + '_ {
        self.0.iter().copied()
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidLength,
    InvalidHint(hint::ParseError),
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
    fn try_from_success(input: &str) {
        assert!(Hints::try_from(input).is_ok());
    }

    #[rstest(input, case(""), case("@"), case("1021"), case("120021"))]
    fn try_from_failure_len(input: &str) {
        assert_eq!(
            Hints::try_from(input).unwrap_err(),
            ParseError::InvalidLength,
        );
    }

    #[rstest(
        input,
        case("1021a"),
        case("10b1@"),
        case("10203"),
        case("00あ12"),
        case("10 20")
    )]
    fn try_from_failure_hint(input: &str) {
        assert!(Hints::try_from(input).is_err());
    }
}
