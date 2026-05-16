use crate::{
    dict::{LETTER_KINDS, LETTERS},
    letter::Letter,
};

#[derive(Debug, Default)]
pub(crate) struct LetterMap<T>([T; LETTER_KINDS]);

impl<T> LetterMap<T> {
    #[cfg(feature = "recommend")]
    pub(crate) fn get(&self, letter: Letter) -> &T {
        &self.0[letter.as_index()]
    }

    pub(crate) fn get_mut(&mut self, letter: Letter) -> &mut T {
        &mut self.0[letter.as_index()]
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Letter, &T)> {
        LETTERS.iter().copied().zip(self.0.iter())
    }
}
