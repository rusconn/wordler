use std::fmt;

use rustc_hash::FxHashSet;

use crate::letter::Letter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Word<'a> {
    pub(crate) str: &'a str,
    pub(crate) letters: Letters,
}

pub(crate) type Letters = FxHashSet<Letter>;

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl<'a> Word<'a> {
    pub(crate) fn from_unchecked(str: &'a str) -> Self {
        Self {
            str,
            letters: str.bytes().map(Letter::from_unchecked).collect(),
        }
    }

    pub(crate) fn unique_letters(&self) -> impl Iterator<Item = Letter> + '_ {
        self.letters.iter().copied()
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
    fn fmt(input: &str, output: &str) {
        assert_eq!(Word::from_unchecked(input).to_string(), output);
    }

    #[rstest(input, output, case("AUDIO", 5), case("HIPPO", 4), case("AAAAA", 1))]
    fn unique_letters(input: &str, output: usize) {
        assert_eq!(Word::from_unchecked(input).unique_letters().count(), output);
    }
}
