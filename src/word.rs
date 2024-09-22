use std::fmt;

use regex::Regex;

use crate::{
    letter::Letter,
    letter_set::{Excludes, Includes, WordLetters},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    word: &'a str,
    letter_set: WordLetters,
}

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> Word<'a> {
    pub fn from_unchecked(str: &'a str) -> Self {
        Self {
            word: str,
            letter_set: WordLetters::from_unchecked(str),
        }
    }

    pub fn unique_letters(&self) -> impl Iterator<Item = Letter> + '_ {
        self.letter_set.iter()
    }

    pub fn is_match(&self, regex: &Regex, includes: &Includes, excludes: &Excludes) -> bool {
        regex.is_match(self.word)
            && includes.is_subset(&self.letter_set)
            && excludes.is_disjoint(&self.letter_set)
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

    #[test]
    fn is_match_regex() {
        let word = Word::from_unchecked("HIPPO");
        let regex = Regex::new("HIPPO").unwrap();
        let includes = Includes::default();
        let excludes = Excludes::default();
        assert!(word.is_match(&regex, &includes, &excludes));

        let regex = Regex::new("ZIPPO").unwrap();
        assert!(!word.is_match(&regex, &includes, &excludes));

        let regex = Regex::new("HIP[^P]O").unwrap();
        assert!(!word.is_match(&regex, &includes, &excludes));
    }

    #[test]
    fn is_match_includes() {
        let word = Word::from_unchecked("HIPPO");
        let regex = Regex::new(".....").unwrap();
        let mut includes = Includes::default();
        let excludes = Excludes::default();

        includes.insert(Letter::from_unchecked(b'I'));
        assert!(word.is_match(&regex, &includes, &excludes));

        includes.insert(Letter::from_unchecked(b'Z'));
        assert!(!word.is_match(&regex, &includes, &excludes));
    }

    #[test]
    fn is_match_excludes() {
        let word = Word::from_unchecked("HIPPO");
        let regex = Regex::new(".....").unwrap();
        let includes = Includes::default();
        let mut excludes = Excludes::default();

        excludes.insert(Letter::from_unchecked(b'Z'));
        assert!(word.is_match(&regex, &includes, &excludes));

        excludes.insert(Letter::from_unchecked(b'P'));
        assert!(!word.is_match(&regex, &includes, &excludes));
    }
}
