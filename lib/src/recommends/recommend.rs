use std::{cmp::Ordering, fmt};

use crate::state::word::Word;

use super::VeiledLetterHistogram;

#[derive(Debug, PartialEq, Eq)]
pub struct Recommend {
    word: &'static Word,
    score: i32,
}

impl fmt::Display for Recommend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl PartialOrd for Recommend {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Recommend {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl Recommend {
    pub(super) fn new(word: &'static Word) -> Self {
        Self { word, score: 0 }
    }

    pub(super) fn update(&mut self, histogram: &VeiledLetterHistogram) {
        self.score = self
            .word
            .letters
            .iter()
            .map(|c| histogram.get(c).unwrap_or(&0))
            .sum()
    }

    pub(super) fn is_useful(&self) -> bool {
        self.score > 0
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{letter::Letter, word::WORDS};

    use super::*;

    fn find_word(str: &str) -> &'static Word {
        let index = WORDS.binary_search_by(|w| w.str.cmp(str)).unwrap();
        &WORDS[index]
    }

    #[rstest(input, output, case("AUDIO", "AUDIO"), case("HIPPO", "HIPPO"))]
    fn fmt(input: &str, output: &str) {
        let word = find_word(input);
        assert_eq!(Recommend::new(word).to_string(), output);
    }

    #[test]
    fn cmp() {
        let a = find_word("AUDIO");
        let b = find_word("HIPPO");
        let mut recommend1 = Recommend::new(a);
        let mut recommend2 = Recommend::new(b);
        assert_eq!(recommend1.cmp(&recommend2), Ordering::Equal);

        recommend1.score = 1;
        assert_eq!(recommend1.cmp(&recommend2), Ordering::Greater);

        recommend2.score = 2;
        assert_eq!(recommend1.cmp(&recommend2), Ordering::Less);
    }

    #[test]
    fn update() {
        let mut histogram: VeiledLetterHistogram = Default::default();

        let hippo = find_word("HIPPO");
        let mut recommend = Recommend::new(hippo);
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
        let hippo = find_word("HIPPO");
        let mut recommend = Recommend::new(hippo);
        assert!(!recommend.is_useful());

        recommend.score = 1;
        assert!(recommend.is_useful());
    }
}
