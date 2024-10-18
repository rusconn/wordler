mod excludes;
mod includes;
mod not_letters;
mod veileds;
mod word_letters;

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use rustc_hash::FxHashSet;

use crate::letter::Letter;

pub use self::{
    excludes::Excludes, includes::Includes, not_letters::NotLetters, veileds::Veileds,
    word_letters::WordLetters,
};

type Set<T> = FxHashSet<T>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetterSet<T>(Set<Letter>, PhantomData<T>);

impl<T> Deref for LetterSet<T> {
    type Target = Set<Letter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for LetterSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> LetterSet<T> {
    pub fn iter(&self) -> impl Iterator<Item = Letter> + '_ {
        self.deref().iter().copied()
    }
}
