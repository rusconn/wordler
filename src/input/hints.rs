mod hint;

use std::{error, fmt, io::Stdin};

use super::util::get_line;

use self::hint::Hint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hints(Vec<Hint>);

impl TryFrom<&str> for Hints {
    type Error = Box<dyn error::Error>;

    fn try_from(hints: &str) -> Result<Self, Self::Error> {
        if hints.chars().count() != 5 {
            return Err(InvalidLengthError.into());
        }

        hints
            .chars()
            .map(Hint::try_from)
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

impl Hints {
    pub fn read(stdin: &Stdin) -> Self {
        loop {
            eprint!("Hints: ");

            let hints = get_line(stdin);

            match Hints::try_from(hints.as_ref()) {
                Ok(hints) => return hints,
                Err(e) => eprintln!("Failed to read the hints: {e}"),
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Hint> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InvalidLengthError;

impl fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hints must be 5 letters")
    }
}

impl error::Error for InvalidLengthError {}
