use std::fmt;

use thiserror::Error;

use wordler::{guess, hints};

#[derive(Debug, Error)]
pub(crate) enum ParseError {
    Guess(#[from] guess::ParseError),
    Hints(#[from] hints::ParseError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ParseError::Guess(e) => show_parse_guess_error(e),
            ParseError::Hints(e) => show_parse_hints_error(e),
        };

        write!(f, "{s}")
    }
}

fn show_parse_guess_error(e: &guess::ParseError) -> String {
    match e {
        guess::ParseError::InvalidLength => "Guess must be 5 letters".into(),
        guess::ParseError::UnknownWord => "Unknown word".into(),
        guess::ParseError::InvalidLetter(c) => format!("Non alphabetical letter: `{c}`"),
    }
}

fn show_parse_hints_error(e: &hints::ParseError) -> String {
    match e {
        hints::ParseError::InvalidLength => "Hints must be 5 letters".into(),
        hints::ParseError::InvalidHint(c) => format!("Invalid hint: `{c}`"),
    }
}
