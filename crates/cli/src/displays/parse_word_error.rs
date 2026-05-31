use std::fmt;

use wordler::ParseWordError;

use crate::AsDisplay;

impl<'a> AsDisplay<'a> for ParseWordError {
    type Target = ParseWordErrorDisplay<'a>;
    fn as_display(&'a self) -> Self::Target {
        ParseWordErrorDisplay(self)
    }
}

pub struct ParseWordErrorDisplay<'a>(pub(crate) &'a ParseWordError);

impl fmt::Display for ParseWordErrorDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ParseWordError::InvalidLength => {
                write!(f, "Guess must be 5 letters")
            }
            ParseWordError::UnknownWord => {
                write!(f, "Unknown word")
            }
            ParseWordError::InvalidLetter(c) => {
                write!(f, "Invalid letter: `{c}`")
            }
        }
    }
}
