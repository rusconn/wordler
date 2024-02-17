mod guess;
mod hints;
mod util;

use std::io::Stdin;

use crate::{Excludes, Includes, LetterInfos, Veileds};

use self::{
    guess::Guess,
    hints::{Hints, Variant as HintVariant},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    guess: Guess,
    hints: Hints,
}

impl Input {
    pub fn read(stdin: &Stdin) -> Self {
        Self {
            guess: Guess::read(stdin),
            hints: Hints::read(stdin),
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
            match hint.0 {
                HintVariant::NotExists => {
                    letter_infos.not(nth, letter);
                    excludes.insert(letter);
                }
                HintVariant::WrongSpot => {
                    letter_infos.not(nth, letter);
                    includes.insert(letter);
                }
                HintVariant::CorrectSpot => {
                    letter_infos.correct(nth, letter);
                    includes.insert(letter);
                }
            }

            veileds.remove(&letter);
        }
    }
}
