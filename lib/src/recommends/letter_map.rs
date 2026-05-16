use crate::{dict::LETTER_KINDS, letter::Letter};

#[derive(Debug, Default)]
pub(super) struct LetterMap<T>([T; LETTER_KINDS]);

impl<T> LetterMap<T> {
    pub(super) fn get(&self, letter: Letter) -> &T {
        &self.0[letter.as_index()]
    }

    pub(super) fn get_mut(&mut self, letter: Letter) -> &mut T {
        &mut self.0[letter.as_index()]
    }
}
