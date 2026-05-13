use thiserror::Error;

use wordler::{InvalidHintError, ParseHintsError, ParseWordError, UpdateStateError};

#[derive(Debug, Error)]
pub enum CliError {
    #[error("{}", show_read_guess_error(.0))]
    ReadGuessError(#[from] ParseWordError),

    #[error("{}", show_read_hints_error(.0))]
    ReadHintsError(#[from] ParseHintsError),

    #[error("{}", show_update_state_error(.0))]
    UpdateStateError(#[from] UpdateStateError),
}

fn show_read_guess_error(e: &ParseWordError) -> String {
    match e {
        ParseWordError::InvalidLength => "Guess must be 5 letters".into(),
        ParseWordError::UnknownWord => "Unknown word".into(),
        ParseWordError::InvalidLetter(c) => format!("Invalid letter: `{c}`"),
    }
}

fn show_read_hints_error(e: &ParseHintsError) -> String {
    match e {
        ParseHintsError::InvalidLength => "Hints must be 5 digits".into(),
        ParseHintsError::InvalidHint(c) => format!("Invalid hint: `{c}`"),
    }
}

fn show_update_state_error(e: &UpdateStateError) -> String {
    match e {
        UpdateStateError::ContradictoryHints(e) => show_invalid_hint_error(e),
    }
}

fn show_invalid_hint_error(e: &InvalidHintError) -> String {
    match e {
        InvalidHintError::Contradictory { letter, hint } => {
            format!("Contradictory hint: (letter = {letter}, hint = {hint})")
        }
    }
}
