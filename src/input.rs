mod guess;
pub mod hints;
mod util;

use std::io::{Stdin, Stdout};

use crate::letter::Letter;

use self::guess::Guess;
pub use self::hints::{Hint, Hints};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    guess: Guess,
    hints: Hints,
}

impl Input {
    pub fn read(stdin: &Stdin, stdout: &mut Stdout) -> Self {
        Self {
            guess: Guess::read(stdin, stdout),
            hints: Hints::read(stdin, stdout),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Letter, Hint)> + '_ {
        self.guess.iter().zip(self.hints.iter())
    }
}
