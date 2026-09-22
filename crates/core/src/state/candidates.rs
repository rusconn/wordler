use crate::dict::WORDS;
use crate::state::constraints::Constraints;

use crate::word::Word;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates(Vec<Word>);

impl Default for Candidates {
    fn default() -> Self {
        Self(WORDS.iter().copied().collect())
    }
}

impl Candidates {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn first(&self) -> Option<Word> {
        self.0.first().copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = Word> {
        self.0.iter().copied()
    }

    pub(crate) fn retain(&mut self, constraints: &Constraints) {
        self.0.retain(|&word| constraints.is_match(word));
    }
}
