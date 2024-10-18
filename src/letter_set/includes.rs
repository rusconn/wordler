use super::LetterSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IncludesDerivative;

pub type Includes = LetterSet<IncludesDerivative>;

impl Default for Includes {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
