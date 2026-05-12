use std::fmt;

use itertools::Itertools;

use wordler::Candidates;

pub struct DisplayCandidates<'a>(&'a Candidates);

impl fmt::Display for DisplayCandidates<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 => write!(f, "Woops, there are no more words"),
            1 => write!(f, "Found: {}", self.0.first().unwrap()),
            n if n <= 50 => write!(f, "Remaining: [{}]", self.0.iter().join(",")),
            n => write!(f, "Remaining: Too many, didn't print: {n}"),
        }
    }
}

impl<'a> super::AsDisplay<'a> for Candidates {
    type Target = DisplayCandidates<'a>;
    fn as_display(&'a self) -> Self::Target {
        DisplayCandidates(self)
    }
}
