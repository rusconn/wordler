mod excludes;
mod includes;
mod not_letters;
mod veileds;
mod word_letters;

use rustc_hash::FxHashSet;

use crate::letter::Letter;

pub use self::{
    excludes::Excludes, includes::Includes, not_letters::NotLetters, veileds::Veileds,
    word_letters::WordLetters,
};

type LetterSet = FxHashSet<Letter>;
