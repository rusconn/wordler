mod dict;
mod hints;
mod letter;
mod letter_map;
mod state;
mod word;

pub use {
    hints::{Hints, ParseError as ParseHintsError},
    state::{Candidates, InvalidHintError, State, UpdateError as UpdateStateError},
    word::{ParseError as ParseWordError, Word},
};

#[cfg(feature = "recommend")]
mod recommends;

#[cfg(feature = "recommend")]
pub use recommends::*;
