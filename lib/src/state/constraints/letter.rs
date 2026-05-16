use crate::dict::WORD_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LetterConstraint {
    min_count: u8,
    max_count: u8,
}

impl Default for LetterConstraint {
    fn default() -> Self {
        Self {
            min_count: 0,
            max_count: WORD_LEN as u8,
        }
    }
}

impl LetterConstraint {
    pub(super) fn update(&mut self, min: u8, max: Option<u8>) -> Result<(), (u8, u8)> {
        if let Some(max) = max
            && max < self.min_count
        {
            return Err((self.min_count, self.max_count));
        }
        if self.max_count < min {
            return Err((self.min_count, self.max_count));
        }

        self.min_count = u8::max(self.min_count, min);
        if let Some(max) = max {
            self.max_count = u8::min(self.max_count, max);
        }

        Ok(())
    }

    pub(super) fn is_match(&self, count: u8) -> bool {
        self.min_count <= count && count <= self.max_count
    }

    pub(super) fn is_active(&self) -> bool {
        0 < self.min_count || self.max_count < WORD_LEN as u8
    }
}
