use anyhow::{bail, Result};

use crate::{Excludes, Includes, Letter, LetterInfos, Veileds};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hint(Variant);

impl TryFrom<char> for Hint {
    type Error = anyhow::Error;

    fn try_from(hint: char) -> Result<Self> {
        match hint {
            '0' => Ok(Self(Variant::NotExists)),
            '1' => Ok(Self(Variant::WrongSpot)),
            '2' => Ok(Self(Variant::CorrectSpot)),
            _ => bail!("Unknown hint: `{hint}`"),
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
enum Variant {
    NotExists,
    WrongSpot,
    CorrectSpot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_success() {
        assert_eq!(Hint::try_from('0').unwrap(), Hint(Variant::NotExists));
        assert_eq!(Hint::try_from('1').unwrap(), Hint(Variant::WrongSpot));
        assert_eq!(Hint::try_from('2').unwrap(), Hint(Variant::CorrectSpot));
    }

    #[test]
    fn try_from_failure() {
        assert_eq!(
            Hint::try_from('@').unwrap_err().to_string(),
            "Unknown hint: `@`"
        );
        assert_eq!(
            Hint::try_from('3').unwrap_err().to_string(),
            "Unknown hint: `3`"
        );
        assert_eq!(
            Hint::try_from('あ').unwrap_err().to_string(),
            "Unknown hint: `あ`"
        );
        assert_eq!(
            Hint::try_from(' ').unwrap_err().to_string(),
            "Unknown hint: ` `"
        );
    }
}
