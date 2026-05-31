mod candidates;
mod recommends;

use std::fmt;

pub use {candidates::*, recommends::*};

pub trait AsDisplay<'a> {
    type Target: fmt::Display;
    fn as_display(&'a self) -> Self::Target;
}
