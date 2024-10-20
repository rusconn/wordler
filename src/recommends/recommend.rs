use std::{cmp::Ordering, fmt};

use crate::word::Word;

use super::VeiledLetterHistogram;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Recommend<'a> {
    word: Word<'a>,
    score: i32,
}

impl<'a> fmt::Display for Recommend<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> PartialOrd for Recommend<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Recommend<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<'a> Recommend<'a> {
    pub(super) fn from_unchecked(str: &'a str) -> Self {
        Self {
            word: Word::from_unchecked(str),
            score: 0,
        }
    }

    pub(super) fn update(&mut self, histogram: &VeiledLetterHistogram) {
        self.score = self
            .word
            .unique_letters()
            .map(|c| histogram.get(&c).unwrap_or(&0))
            .sum()
    }

    pub(super) fn is_useful(&self) -> bool {
        self.score > 0
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::letter::Letter;

    use super::*;

    #[rstest(
        input,
        output,
        case("AUDIO", "AUDIO"),
        case("HIPPO", "HIPPO"),
        case("AAAAA", "AAAAA")
    )]
    fn fmt(input: &str, output: &str) {
        assert_eq!(Recommend::from_unchecked(input).to_string(), output);
    }

    #[test]
    fn cmp() {
        let mut recommend1 = Recommend::from_unchecked("AAAAA");
        let mut recommend2 = Recommend::from_unchecked("BBBBB");
        assert_eq!(recommend1.cmp(&recommend2), Ordering::Equal);

        recommend1.score = 1;
        assert_eq!(recommend1.cmp(&recommend2), Ordering::Greater);

        recommend2.score = 2;
        assert_eq!(recommend1.cmp(&recommend2), Ordering::Less);
    }

    #[test]
    fn update() {
        let mut histogram: VeiledLetterHistogram = Default::default();

        let mut recommend = Recommend::from_unchecked("HIPPO");
        assert_eq!(recommend.score, 0);

        *histogram.entry(Letter::from_unchecked(b'A')).or_insert(0) += 1;
        recommend.update(&histogram);
        assert_eq!(recommend.score, 0);

        *histogram.entry(Letter::from_unchecked(b'P')).or_insert(0) += 1;
        recommend.update(&histogram);
        assert_eq!(recommend.score, 1);

        *histogram.entry(Letter::from_unchecked(b'I')).or_insert(0) += 1;
        recommend.update(&histogram);
        assert_eq!(recommend.score, 2);

        *histogram.entry(Letter::from_unchecked(b'I')).or_insert(0) += 1;
        recommend.update(&histogram);
        assert_eq!(recommend.score, 3);
    }

    #[test]
    fn is_useful() {
        let mut recommend = Recommend::from_unchecked("HIPPO");
        assert!(!recommend.is_useful());

        recommend.score = 1;
        assert!(recommend.is_useful());
    }
}
