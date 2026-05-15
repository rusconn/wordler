use std::sync::LazyLock;

use rustc_hash::FxHashSet;

use crate::{dict::WORD_STRINGS, letter::Letter};

pub(super) static WORD_LETTER_SETS: LazyLock<Vec<FxHashSet<Letter>>> = LazyLock::new(|| {
    WORD_STRINGS
        .iter()
        .map(|word| FxHashSet::from_iter(word.bytes().map(Letter::from_unchecked)))
        .collect()
});
