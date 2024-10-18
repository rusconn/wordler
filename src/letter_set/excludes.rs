use super::LetterSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExcludesDerivative;

pub type Excludes = LetterSet<ExcludesDerivative>;

impl Default for Excludes {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
