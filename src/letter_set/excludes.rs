use std::ops::{Deref, DerefMut};

use super::LetterSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Excludes(LetterSet);

impl Deref for Excludes {
    type Target = LetterSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Excludes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
