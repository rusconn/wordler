mod guess;
mod hints;
mod util;

use std::io::{Stdin, Stdout};

use crate::letter::Letter;

use self::guess::Guess;
pub(crate) use self::hints::Hint;
use self::hints::Hints;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Input {
    guess: Guess,
    hints: Hints,
}

impl Input {
    pub(crate) fn read(stdin: &Stdin, stdout: &mut Stdout) -> Self {
        Self {
            guess: Guess::read(stdin, stdout),
            hints: Hints::read(stdin, stdout),
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Letter, Hint)> + '_ {
        self.guess.iter().zip(self.hints.iter())
    }
}
