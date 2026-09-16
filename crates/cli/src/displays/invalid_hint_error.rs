use std::fmt;

use wordler::InvalidHintError;

use crate::AsDisplay;

impl AsDisplay for InvalidHintError {
    type Target<'a> = InvalidHintErrorDisplay<'a>;
    fn as_display(&self) -> Self::Target<'_> {
        InvalidHintErrorDisplay(self)
    }
}

pub struct InvalidHintErrorDisplay<'a>(&'a InvalidHintError);

impl fmt::Display for InvalidHintErrorDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            InvalidHintError::Contradictory { letter, hint } => {
                write!(f, "Contradictory hint: (letter = {letter}, hint = {hint})")
            }
        }
    }
}
