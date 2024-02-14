use std::{collections::HashSet, ops::Index};

use regex::Regex;

use crate::{letter_infos::LetterInfos, word::Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates<'a> {
    words: Vec<Word<'a>>,
}

impl<'a, const N: usize> From<&[&'a str; N]> for Candidates<'a> {
    fn from(words: &[&'a str; N]) -> Self {
        Self {
            words: words.iter().map(|&word| Word::from(word)).collect(),
        }
    }
}

impl<'a> Index<usize> for Candidates<'a> {
    type Output = Word<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.words[index]
    }
}

impl<'a> Candidates<'a> {
    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Word<'a>> {
        self.words.iter()
    }

    pub fn retain(
        &mut self,
        letter_infos: &LetterInfos,
        contains: &HashSet<char>,
        not_contains: &HashSet<char>,
    ) {
        let regex = Regex::new(&letter_infos.as_regex())
            .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"));

        self.words
            .retain(|word| word.is_match(&regex, &contains, &not_contains));
    }
}
