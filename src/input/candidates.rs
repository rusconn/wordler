use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashSet;

use crate::{dict::WORDS, input::letter_info::LetterInfo, letter::Letter, word::Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates<'a>(Vec<Word<'a>>);

impl Default for Candidates<'_> {
    fn default() -> Self {
        Self(WORDS.into_iter().map(Word::from_unchecked).collect())
    }
}

impl<'a> Candidates<'a> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn first(&self) -> Option<&Word<'a>> {
        self.0.first()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Word<'a>> {
        self.0.iter()
    }

    pub(crate) fn retain(
        &mut self,
        infos: &[LetterInfo],
        includes: &FxHashSet<Letter>,
        excludes: &FxHashSet<Letter>,
    ) {
        let regex = Self::regex(infos);
        self.0.retain(|word| {
            regex.is_match(word.str)
                && includes.is_subset(&word.letters)
                && excludes.is_disjoint(&word.letters)
        });
    }

    fn regex(infos: &[LetterInfo]) -> Regex {
        Regex::new(Self::regex_string(infos).as_str())
            .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"))
    }

    fn regex_string(infos: &[LetterInfo]) -> String {
        infos.iter().map(LetterInfo::regex_string).join("")
    }
}
