mod hint;

use std::io::Stdin;

use anyhow::{ensure, Result};

use super::util::get_line;

use self::hint::Hint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hints(Vec<Hint>);

impl TryFrom<&str> for Hints {
    type Error = anyhow::Error;

    fn try_from(hints: &str) -> Result<Self> {
        ensure!(hints.chars().count() == 5, "Hints must be 5 letters");

        hints
            .chars()
            .map(Hint::try_from)
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

impl Hints {
    pub fn read(stdin: &Stdin) -> Self {
        loop {
            eprint!("Hints: ");

            let hints = get_line(stdin);

            match Hints::try_from(hints.as_ref()) {
                Ok(hints) => return hints,
                Err(e) => eprintln!("Failed to read the hints: {e}"),
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Hint> + '_ {
        self.0.iter().copied()
    }
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
            Hints::try_from(input).unwrap_err().to_string(),
            "Hints must be 5 letters"
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
