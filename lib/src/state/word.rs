use std::fmt;

use rustc_hash::FxHashSet;

use crate::letter::Letter;

#[derive(Debug, PartialEq, Eq)]
pub struct Word {
    pub(crate) str: &'static str,
    pub(crate) letters: FxHashSet<Letter>,
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
