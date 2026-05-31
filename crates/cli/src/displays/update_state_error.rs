use std::fmt;

use wordler::UpdateStateError;

use super::AsDisplay;

impl<'a> AsDisplay<'a> for UpdateStateError {
    type Target = UpdateStateErrorDisplay<'a>;
    fn as_display(&'a self) -> Self::Target {
        UpdateStateErrorDisplay(self)
    }
}

pub struct UpdateStateErrorDisplay<'a>(&'a UpdateStateError);

impl fmt::Display for UpdateStateErrorDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            UpdateStateError::ContradictoryPosition(e) => {
                write!(f, "{}", e.as_display())
            }
            UpdateStateError::ContradictoryCount { letter, min, max } => {
                write!(
                    f,
                    "Contradictory count: letter={letter}, min={min}, max={max}"
                )
            }
        }
    }
}
