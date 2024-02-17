use std::{collections::HashSet, ops::Index};

use regex::Regex;

use crate::{LetterInfos, Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates<'a>(Vec<Word<'a>>);

impl<'a, const N: usize> From<&[&'a str; N]> for Candidates<'a> {
    fn from(words: &[&'a str; N]) -> Self {
        Self(words.iter().map(|&word| Word::from(word)).collect())
    }
}

impl<'a> Index<usize> for Candidates<'a> {
    type Output = Word<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> Candidates<'a> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Word<'a>> {
        self.0.iter()
    }

    pub fn retain(
        &mut self,
        letter_infos: &LetterInfos,
        contains: &HashSet<char>,
        not_contains: &HashSet<char>,
    ) {
        let regex = Regex::new(&letter_infos.as_regex())
            .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"));

        self.0
            .retain(|word| word.is_match(&regex, contains, not_contains));
    }
}
