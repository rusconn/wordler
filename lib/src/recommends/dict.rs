use std::sync::LazyLock;

use crate::{dict::WORD_STRINGS, letter::Letter};

use super::letter_set::LetterSet;

pub(super) static WORD_LETTER_SETS: LazyLock<Vec<LetterSet>> = LazyLock::new(|| {
    WORD_STRINGS
        .iter()
        .map(|word| {
            let mut set = LetterSet::default();
            for byte in word.bytes() {
                set.insert(Letter::from_unchecked(byte));
            }
            set
        })
        .collect()
});
