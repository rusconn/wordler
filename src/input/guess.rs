use std::io::Stdin;

use anyhow::{ensure, Result};

use crate::letter::Letter;

use super::util::get_line;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guess(Vec<Letter>);

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
    pub fn read(stdin: &Stdin) -> Self {
        loop {
            eprint!("Guess: ");

            let guess = get_line(stdin);

            match Self::try_from(guess.as_ref()) {
                Ok(guess) => return guess,
                Err(e) => eprintln!("Failed to read the guess: {e}"),
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.0.iter().copied()
    }
}
