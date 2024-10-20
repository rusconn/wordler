use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in super::super) enum Hint {
    NotExists,
    WrongSpot,
    CorrectSpot,
}

impl TryFrom<char> for Hint {
    type Error = anyhow::Error;

    fn try_from(hint: char) -> Result<Self> {
        match hint {
            '0' => Ok(Self::NotExists),
            '1' => Ok(Self::WrongSpot),
            '2' => Ok(Self::CorrectSpot),
            _ => bail!("Unknown hint: `{hint}`"),
        }
    }
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
            Hint::try_from(input).unwrap_err().to_string(),
            format!("Unknown hint: `{input}`"),
        );
    }
}
