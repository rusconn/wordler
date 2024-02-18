use std::{error, fmt};

use crate::{Excludes, Includes, Letter, LetterInfos, Veileds};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hint(Variant);

impl TryFrom<char> for Hint {
    type Error = Box<dyn error::Error>;

    fn try_from(hint: char) -> Result<Self, Self::Error> {
        match hint {
            '0' => Ok(Self(Variant::NotExists)),
            '1' => Ok(Self(Variant::WrongSpot)),
            '2' => Ok(Self(Variant::CorrectSpot)),
            _ => Err(UnknownHintError(hint).into()),
        }
    }
}

impl Hint {
    pub fn apply(
        &self,
        nth: usize,
        letter: Letter,
        letter_infos: &mut LetterInfos,
        includes: &mut Includes,
        excludes: &mut Excludes,
        veileds: &mut Veileds,
    ) {
        match self.0 {
            Variant::NotExists => {
                letter_infos.not(nth, letter);
                excludes.insert(letter);
            }
            Variant::WrongSpot => {
                letter_infos.not(nth, letter);
                includes.insert(letter);
            }
            Variant::CorrectSpot => {
                letter_infos.correct(nth, letter);
                includes.insert(letter);
            }
        }

        veileds.remove(letter);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnknownHintError(char);

impl fmt::Display for UnknownHintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown hint: `{}`", self.0)
    }
}

impl error::Error for UnknownHintError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Variant {
    NotExists,
    WrongSpot,
    CorrectSpot,
}
