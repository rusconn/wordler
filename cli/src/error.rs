use thiserror::Error;

use wordler::{ParseHintsError, ParseWordError};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{}", show_parse_word_error(.0))]
    Guess(#[from] ParseWordError),

    #[error("{}", show_parse_hints_error(.0))]
    Hints(#[from] ParseHintsError),
}

fn show_parse_word_error(e: &ParseWordError) -> String {
    match e {
        ParseWordError::InvalidLength => "Guess must be 5 letters".into(),
        ParseWordError::UnknownWord => "Unknown word".into(),
        ParseWordError::InvalidLetter(c) => format!("Non alphabetical letter: `{c}`"),
    }
}

fn show_parse_hints_error(e: &ParseHintsError) -> String {
    match e {
        ParseHintsError::InvalidLength => "Hints must be 5 letters".into(),
        ParseHintsError::InvalidHint(c) => format!("Invalid hint: `{c}`"),
    }
}
