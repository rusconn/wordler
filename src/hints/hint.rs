use std::collections::HashSet;

use wordler::LetterInfos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hint(Variant);

impl TryFrom<char> for Hint {
    type Error = InvalidHintError;

    fn try_from(hint: char) -> Result<Self, Self::Error> {
        match hint {
            '0' => Ok(Self(Variant::NotExists)),
            '1' => Ok(Self(Variant::WrongSpot)),
            '2' => Ok(Self(Variant::CorrectSpot)),
            _ => Err(Self::Error::UnknownHint(hint)),
        }
    }
}

impl Hint {
    pub fn apply(
        &self,
        nth: usize,
        letter: char,
        letter_infos: &mut LetterInfos,
        contains: &mut HashSet<char>,
        not_contains: &mut HashSet<char>,
        unuseds: &mut HashSet<char>,
    ) {
        match self.0 {
            Variant::NotExists => {
                letter_infos.not(nth, letter);
                not_contains.insert(letter);
            }
            Variant::WrongSpot => {
                letter_infos.not(nth, letter);
                contains.insert(letter);
            }
            Variant::CorrectSpot => {
                letter_infos.correct(nth, letter);
                contains.insert(letter);
            }
        }

        unuseds.remove(&letter);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidHintError {
    UnknownHint(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Variant {
    NotExists,
    WrongSpot,
    CorrectSpot,
}
