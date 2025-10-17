use std::fmt;

use thiserror::Error;

use wordler::{ParseGuessError, ParseHintError, ParseHintsError, ParseLetterError};

#[derive(Debug, Error)]
pub(crate) enum ParseError {
    Guess(#[from] ParseGuessError),
    Hints(#[from] ParseHintsError),
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

fn show_parse_guess_error(e: &ParseGuessError) -> String {
    match e {
        ParseGuessError::InvalidLength => "Guess must be 5 letters".into(),
        ParseGuessError::UnknownWord => "Unknown word".into(),
        ParseGuessError::InvalidLetter(e) => show_parse_letter_error(e),
    }
}

fn show_parse_letter_error(e: &ParseLetterError) -> String {
    match e {
        ParseLetterError::NonAlphabeticalLetter(c) => {
            format!("Non alphabetical letter: `{c}`")
        }
    }
}

fn show_parse_hints_error(e: &ParseHintsError) -> String {
    match e {
        ParseHintsError::InvalidLength => "Hints must be 5 letters".into(),
        ParseHintsError::InvalidHint(e) => show_parse_hint_error(e),
    }
}

fn show_parse_hint_error(e: &ParseHintError) -> String {
    match e {
        ParseHintError::InvalidHint(c) => format!("Invalid hint: `{c}`"),
    }
}
