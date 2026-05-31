use std::fmt;

use wordler::ParseHintsError;

use crate::AsDisplay;

impl<'a> AsDisplay<'a> for ParseHintsError {
    type Target = ParseHintsErrorDisplay<'a>;
    fn as_display(&'a self) -> Self::Target {
        ParseHintsErrorDisplay(self)
    }
}

pub struct ParseHintsErrorDisplay<'a>(&'a ParseHintsError);

impl fmt::Display for ParseHintsErrorDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ParseHintsError::InvalidLength => {
                write!(f, "Hints must be 5 digits")
            }
            ParseHintsError::InvalidHint(c) => {
                write!(f, "Invalid hint: `{c}`")
            }
        }
    }
}
