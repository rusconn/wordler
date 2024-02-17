mod guess;
mod hints;
mod util;

use std::{collections::HashSet, io::Stdin};

use crate::LetterInfos;

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
        contains: &mut HashSet<char>,
        not_contains: &mut HashSet<char>,
        unuseds: &mut HashSet<char>,
    ) {
        for (nth, (letter, hint)) in self.guess.iter().zip(self.hints.iter()).enumerate() {
            match hint.0 {
                HintVariant::NotExists => {
                    letter_infos.not(nth, letter.as_char());
                    not_contains.insert(letter.as_char());
                }
                HintVariant::WrongSpot => {
                    letter_infos.not(nth, letter.as_char());
                    contains.insert(letter.as_char());
                }
                HintVariant::CorrectSpot => {
                    letter_infos.correct(nth, letter.as_char());
                    contains.insert(letter.as_char());
                }
            }

            unuseds.remove(&letter.as_char());
        }
    }
}
