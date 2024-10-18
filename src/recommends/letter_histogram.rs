use std::ops::{Deref, DerefMut};

use rustc_hash::FxHashMap;

use crate::letter::Letter;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LetterHistogram(FxHashMap<Letter, i32>);

impl Deref for LetterHistogram {
    type Target = FxHashMap<Letter, i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LetterHistogram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
