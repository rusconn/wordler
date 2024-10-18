use std::ops::{Deref, DerefMut};

use super::LetterSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Includes(LetterSet);

impl Deref for Includes {
    type Target = LetterSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Includes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
