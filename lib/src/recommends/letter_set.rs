use crate::letter::Letter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct LetterSet(u32);

impl LetterSet {
    pub(super) fn insert(&mut self, letter: Letter) {
        self.0 |= 1 << letter.as_index();
    }

    pub(super) fn letters(&self) -> Letters {
        Letters(self.0)
    }
}

pub(super) struct Letters(u32);

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
