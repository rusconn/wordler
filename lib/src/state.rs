mod candidates;
mod constraints;

use crate::word::Word;

use super::hints::Hints;

use constraints::Constraints;

pub use {
    candidates::Candidates,
    constraints::{CheckPositionError as InvalidHintError, UpdateError},
};

#[cfg_attr(test, derive(Clone, PartialEq, Eq))]
#[derive(Debug, Default)]
pub struct State {
    constraints: Constraints,
    candidates: Candidates,
}

impl State {
    pub fn update(&mut self, guess: &Word, hints: &Hints) -> Result<(), UpdateError> {
        self.constraints.update(*guess, hints)?;
        self.candidates.retain(&self.constraints);
        Ok(())
    }

    pub(crate) fn constraints(&self) -> &Constraints {
        &self.constraints
    }

    pub fn candidates(&self) -> &Candidates {
        &self.candidates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_contradiction() {
        let mut state = State::default();

        let guess = Word::from_unchecked(0);
        let hints = "20000".parse::<Hints>().unwrap();
        state.update(&guess, &hints).unwrap();

        let hints = "00000".parse::<Hints>().unwrap(); // 0th: 2?0?
        assert!(matches!(
            state.update(&guess, &hints),
            Err(UpdateError::ContradictoryPosition(_))
        ));
    }

    #[test]
    fn update_atomicity() {
        let mut state = State::default();

        let guess = Word::from_unchecked(69); // "ABUSE"
        let hints = "00002".parse::<Hints>().unwrap();
        state.update(&guess, &hints).unwrap();

        let guess = Word::from_unchecked(122); // "ACUTE"
        let hints = "01021".parse::<Hints>().unwrap(); // E: 2?1?
        let backup = state.clone();
        let _ = state.update(&guess, &hints).unwrap_err();
        assert_eq!(backup, state);
    }
}
