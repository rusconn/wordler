use derive_more::derive::{Deref, DerefMut};

use super::LetterSet;

#[derive(Debug, Clone, PartialEq, Eq, Default, Deref, DerefMut)]
pub struct Includes(LetterSet);
