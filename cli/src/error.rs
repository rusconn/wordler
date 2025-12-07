use thiserror::Error;

use wordler::{guess, hints};

#[derive(Debug, Error)]
pub(crate) enum ParseError {
    #[error("{}", show_parse_guess_error(.0))]
    Guess(#[from] guess::ParseError),

    #[error("{}", show_parse_hints_error(.0))]
    Hints(#[from] hints::ParseError),
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
