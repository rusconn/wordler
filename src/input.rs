mod guess;
mod hints;
mod util;

use std::io::{Stdin, Stdout};

use crate::{Excludes, Includes, LetterInfos, Veileds};

use self::{guess::Guess, hints::Hints};

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

    pub fn apply(
        &self,
        letter_infos: &mut LetterInfos,
        includes: &mut Includes,
        excludes: &mut Excludes,
        veileds: &mut Veileds,
    ) {
        for (nth, (letter, hint)) in self.guess.iter().zip(self.hints.iter()).enumerate() {
            hint.apply(nth, letter, letter_infos, includes, excludes, veileds);
        }
    }
}
