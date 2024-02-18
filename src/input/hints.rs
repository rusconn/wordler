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
