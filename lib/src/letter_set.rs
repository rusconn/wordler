use crate::letter::Letter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct LetterSet(u32);

impl LetterSet {
    pub(crate) fn insert(&mut self, letter: Letter) {
        self.0 |= 1 << letter.as_index();
    }

    pub(crate) fn contains(&self, letter: Letter) -> bool {
        (self.0 & (1 << letter.as_index())) != 0
    }

    #[cfg(feature = "recommend")]
    pub(crate) fn letters(&self) -> Letters {
        Letters(self.0)
    }
}

#[cfg(feature = "recommend")]
pub(crate) struct Letters(u32);

#[cfg(feature = "recommend")]
impl Iterator for Letters {
    type Item = Letter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let index = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Some(Letter::from_unchecked(b'A' + index))
    }
}
