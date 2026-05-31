use std::fmt;

use itertools::Itertools;

use wordler::Recommends;

pub struct DisplayRecommends<'a>(&'a Recommends);

impl fmt::Display for DisplayRecommends<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "Recommend: -")
        } else {
            write!(f, "Recommend: [{}]", self.0.iter().take(5).join(","))
        }
    }
}

impl<'a> super::AsDisplay<'a> for Recommends {
    type Target = DisplayRecommends<'a>;
    fn as_display(&'a self) -> Self::Target {
        DisplayRecommends(self)
    }
}
