use thiserror::Error;

use wordler::{ParseGuessError, ParseHintsError};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{}", show_parse_guess_error(.0))]
    Guess(#[from] ParseGuessError),

    #[error("{}", show_parse_hints_error(.0))]
    Hints(#[from] ParseHintsError),
}

fn show_parse_guess_error(e: &ParseGuessError) -> String {
    match e {
        ParseGuessError::InvalidLength => "Guess must be 5 letters".into(),
        ParseGuessError::UnknownWord => "Unknown word".into(),
        ParseGuessError::InvalidLetter(c) => format!("Non alphabetical letter: `{c}`"),
    }
}

fn show_parse_hints_error(e: &ParseHintsError) -> String {
    match e {
        ParseHintsError::InvalidLength => "Hints must be 5 letters".into(),
        ParseHintsError::InvalidHint(c) => format!("Invalid hint: `{c}`"),
    }
}
