use std::fmt;

use regex::Regex;
use rustc_hash::FxHashSet;

use crate::{Excludes, Includes, Letter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    word: &'a str,
    letter_set: FxHashSet<Letter>,
}

impl<'a> fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl<'a> Word<'a> {
    /// Make sure the str is valid
    pub fn unsafe_from(str: &'a str) -> Self {
        Self {
            word: str,
            letter_set: FxHashSet::from_iter(str.bytes().map(Letter::unsafe_from)),
        }
    }

    pub fn unique_letters(&self) -> impl Iterator<Item = Letter> + '_ {
        self.letter_set.iter().copied()
    }

    pub fn is_match(&self, regex: &Regex, includes: &Includes, excludes: &Excludes) -> bool {
        regex.is_match(self.word)
            && includes.is_subset(&self.letter_set)
            && excludes.is_disjoint(&self.letter_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt() {
        assert_eq!(Word::unsafe_from("AUDIO").to_string(), "AUDIO");
        assert_eq!(Word::unsafe_from("HIPPO").to_string(), "HIPPO");
        assert_eq!(Word::unsafe_from("AAAAA").to_string(), "AAAAA");
    }

    #[test]
    fn unique_letters() {
        assert_eq!(Word::unsafe_from("AUDIO").unique_letters().count(), 5);
        assert_eq!(Word::unsafe_from("HIPPO").unique_letters().count(), 4);
        assert_eq!(Word::unsafe_from("AAAAA").unique_letters().count(), 1);
    }

    #[test]
    fn is_match_regex() {
        let word = Word::unsafe_from("HIPPO");
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
        let word = Word::unsafe_from("HIPPO");
        let regex = Regex::new(".....").unwrap();
        let mut includes = Includes::default();
        let excludes = Excludes::default();

        includes.insert(Letter::unsafe_from(b'I'));
        assert!(word.is_match(&regex, &includes, &excludes));

        includes.insert(Letter::unsafe_from(b'Z'));
        assert!(!word.is_match(&regex, &includes, &excludes));
    }

    #[test]
    fn is_match_excludes() {
        let word = Word::unsafe_from("HIPPO");
        let regex = Regex::new(".....").unwrap();
        let includes = Includes::default();
        let mut excludes = Excludes::default();

        excludes.insert(Letter::unsafe_from(b'Z'));
        assert!(word.is_match(&regex, &includes, &excludes));

        excludes.insert(Letter::unsafe_from(b'P'));
        assert!(!word.is_match(&regex, &includes, &excludes));
    }
}
