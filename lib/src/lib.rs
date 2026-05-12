mod dict;
mod input;
mod letter;
mod state;

pub use {input::*, state::*};

#[cfg(feature = "recommend")]
mod recommends;

#[cfg(feature = "recommend")]
pub use recommends::*;
