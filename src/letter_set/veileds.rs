use derive_more::derive::{Deref, DerefMut};

use super::{Letter, LetterSet};

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct Veileds(LetterSet);

impl Default for Veileds {
    fn default() -> Self {
        Self((b'A'..=b'Z').map(Letter::from_unchecked).collect())
    }
}
