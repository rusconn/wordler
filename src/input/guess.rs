use std::io::{Stdin, Stdout, Write};

use anyhow::{Result, ensure};

use crate::letter::Letter;

use super::util::get_line;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Guess(Vec<Letter>);

impl TryFrom<&str> for Guess {
    type Error = anyhow::Error;

    fn try_from(guess: &str) -> Result<Self> {
        ensure!(guess.chars().count() == 5, "Guess must be 5 letters");

        guess
            .chars()
            .map(Letter::try_from)
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

impl Guess {
    pub(super) fn read(stdin: &Stdin, stdout: &mut Stdout) -> Self {
        loop {
            print!("Guess: ");
            stdout.flush().expect("Failed to write to stdout");

            let guess = get_line(stdin);

            match Self::try_from(guess.as_ref()) {
                Ok(guess) => return guess,
                Err(e) => eprintln!("Failed to read the guess: {e}"),
            }
        }
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.0.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(input, case("audio"), case("STERN"), case("cHuMp"), case("aaaaa"))]
    fn try_from_success(input: &str) {
        assert!(Guess::try_from(input).is_ok());
    }

    #[rstest(input, case(""), case("@"), case("will"), case("clippy"))]
    fn try_from_failure_len(input: &str) {
        assert_eq!(
            Guess::try_from(input).unwrap_err().to_string(),
            "Guess must be 5 letters"
        );
    }

    #[rstest(input, case("will@"), case("1will"), case("wiあll"), case("wi ll"))]
    fn try_from_failure_letter(input: &str) {
        assert!(Guess::try_from(input).is_err());
    }
}
