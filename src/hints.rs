mod hint;

use std::collections::HashSet;

use wordler::LetterInfos;

use crate::Guess;

use self::hint::{Hint, UnknownHintError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hints(Vec<Hint>);

impl TryFrom<&str> for Hints {
    type Error = UnknownHintError;

    fn try_from(hints: &str) -> Result<Self, Self::Error> {
        hints
            .chars()
            .map(Hint::try_from)
            .collect::<Result<_, _>>()
            .map(Hints)
    }
}

impl Hints {
    pub fn apply(
        &self,
        guess: &Guess,
        letter_infos: &mut LetterInfos,
        contains: &mut HashSet<char>,
        not_contains: &mut HashSet<char>,
        unuseds: &mut HashSet<char>,
    ) {
        for (nth, (letter, hint)) in guess.letters().zip(&self.0).enumerate() {
            hint.apply(nth, letter, letter_infos, contains, not_contains, unuseds);
        }
    }
}
