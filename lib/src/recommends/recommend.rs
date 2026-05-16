use std::{cmp::Ordering, fmt};

use crate::word::Word;

use super::LetterHistogram;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub(super) fn update(&mut self, histogram: &LetterHistogram, candidates_count: i32) {
        self.score = self
            .word
            .as_letter_set()
            .letters()
            .map(|letter| {
                let occurrence = *histogram.get(letter);
                i32::min(occurrence, candidates_count - occurrence)
            })
            .sum()
    }

    pub(super) fn is_useful(&self) -> bool {
        self.score > 0
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::dict::{WORD_STRINGS, WORDS};

    use super::*;

    fn find_word(str: &str) -> &'static Word {
        let index = WORD_STRINGS.binary_search_by(|ws| ws.cmp(&str)).unwrap();
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
    fn is_useful() {
        let hippo = find_word("HIPPO");
        let mut recommend = Recommend::new(hippo);
        assert!(!recommend.is_useful());

        recommend.score = 1;
        assert!(recommend.is_useful());
    }
}
