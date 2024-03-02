use anyhow::{bail, Result};

use crate::{
    letter::Letter,
    letter_infos::LetterInfos,
    letter_set::{Excludes, Includes, Veileds},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant {
    NotExists,
    WrongSpot,
    CorrectSpot,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Variant::*;
    use super::*;

    #[rstest(
        input,
        variant,
        case('0', NotExists),
        case('1', WrongSpot),
        case('2', CorrectSpot)
    )]
    fn try_from_success(input: char, variant: Variant) {
        assert_eq!(Hint::try_from(input).unwrap(), Hint(variant));
    }

    #[rstest(input, case('@'), case('3'), case('あ'), case(' '))]
    fn try_from_failure(input: char) {
        assert_eq!(
            Hint::try_from(input).unwrap_err().to_string(),
            format!("Unknown hint: `{input}`"),
        );
    }
}
