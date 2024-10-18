use derive_more::derive::{Deref, DerefMut};
use rustc_hash::FxHashMap;

use crate::letter::Letter;

#[derive(Debug, Clone, PartialEq, Eq, Default, Deref, DerefMut)]
pub struct LetterHistogram(FxHashMap<Letter, i32>);
