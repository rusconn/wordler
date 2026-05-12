use regex::Regex;
use rustc_hash::FxHashSet;

use crate::{
    dict::WORDS,
    letter::Letter,
    state::{letter_info::LetterInfo, to_regex_string::ToRegexString},
};

use crate::word::Word;

#[derive(Debug, PartialEq, Eq)]
pub struct Candidates(Vec<&'static Word>);

impl Default for Candidates {
    fn default() -> Self {
        Self(WORDS.iter().collect())
    }
}

impl Candidates {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn first(&self) -> Option<&'static Word> {
        self.0.first().copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static Word> {
        self.0.iter().copied()
    }

    pub(crate) fn retain(
        &mut self,
        infos: &[LetterInfo],
        includes: &FxHashSet<Letter>,
        excludes: &FxHashSet<Letter>,
    ) {
        let regex = Regex::new(&infos.to_regex_string()) //
            .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"));

        self.0.retain(|word| {
            regex.is_match(word.as_str())
                && includes.is_subset(word.as_letter_set())
                && excludes.is_disjoint(word.as_letter_set())
        });
    }
}
