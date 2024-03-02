use crate::letter::Letter;

use super::{LetterSet, Set};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VeiledsDerivative;

pub type Veileds = LetterSet<VeiledsDerivative>;

impl Default for Veileds {
    fn default() -> Self {
        Self(
            Set::from_iter((b'A'..=b'Z').map(Letter::unsafe_from)),
            Default::default(),
        )
    }
}

impl Veileds {
    pub fn contains(&self, letter: Letter) -> bool {
        self.0.contains(&letter)
    }

    pub fn remove(&mut self, letter: Letter) -> bool {
        self.0.remove(&letter)
    }
}
