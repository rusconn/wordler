mod candidates;
mod invalid_hint_error;
mod parse_hints_error;
mod parse_word_error;
mod recommends;
mod update_state_error;

use std::fmt;

pub use {
    candidates::*, invalid_hint_error::*, parse_hints_error::*, parse_word_error::*, recommends::*,
    update_state_error::*,
};

pub trait AsDisplay<'a> {
    type Target: fmt::Display;
    fn as_display(&'a self) -> Self::Target;
}
