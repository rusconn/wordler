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

    pub fn iter(&self) -> impl Iterator<Item = &Hint> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_success() {
        assert!(Hints::try_from("00000").is_ok());
        assert!(Hints::try_from("11111").is_ok());
        assert!(Hints::try_from("22222").is_ok());
        assert!(Hints::try_from("01010").is_ok());
        assert!(Hints::try_from("01201").is_ok());
    }

    #[test]
    fn try_from_failure_len() {
        assert_eq!(
            Hints::try_from("").unwrap_err().to_string(),
            "Hints must be 5 letters"
        );
        assert_eq!(
            Hints::try_from("@").unwrap_err().to_string(),
            "Hints must be 5 letters"
        );
        assert_eq!(
            Hints::try_from("1021").unwrap_err().to_string(),
            "Hints must be 5 letters"
        );
        assert_eq!(
            Hints::try_from("120021").unwrap_err().to_string(),
            "Hints must be 5 letters"
        );
    }

    #[test]
    fn try_from_failure_hint() {
        assert!(Hints::try_from("1021a").is_err());
        assert!(Hints::try_from("10b1@").is_err());
        assert!(Hints::try_from("10203").is_err());
        assert!(Hints::try_from("00あ12").is_err());
        assert!(Hints::try_from("10 20").is_err());
    }
}
