use std::{cmp::Ordering, fmt, sync::LazyLock};

use rustc_hash::FxHashSet;

use crate::{dict, letter::Letter};

pub(crate) static WORDS: LazyLock<Vec<Word>> = LazyLock::new(|| {
    dict::WORDS
        .iter()
        .map(|&s| Word::from_unchecked(s))
        .collect()
});

#[derive(Debug, PartialEq, Eq)]
pub struct Word {
    pub(crate) str: &'static str,
    pub(crate) letters: FxHashSet<Letter>,
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        self.str.cmp(other.str)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl Word {
    pub(crate) fn from_unchecked(str: &'static str) -> Self {
        Self {
            str,
            letters: str.bytes().map(Letter::from_unchecked).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input,
        output,
        case("AUDIO", "AUDIO"),
        case("HIPPO", "HIPPO"),
        case("AAAAA", "AAAAA")
    )]
    fn fmt(input: &'static str, output: &str) {
        assert_eq!(Word::from_unchecked(input).to_string(), output);
    }

    #[rstest(input, output, case("AUDIO", 5), case("HIPPO", 4), case("AAAAA", 1))]
    fn unique_letters(input: &'static str, output: usize) {
        assert_eq!(Word::from_unchecked(input).letters.len(), output);
    }
}
