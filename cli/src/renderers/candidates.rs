use itertools::Itertools;

use wordler::Candidates;

pub fn render(candidates: &Candidates) -> String {
    match candidates.len() {
        0 => "Woops, there are no more words".into(),
        1 => format!("Found: {}", candidates.first().unwrap()),
        n if n <= 50 => format!("Remaining: [{}]", candidates.iter().join(",")),
        n => format!("Remaining: Too many, didn't print: {n}"),
    }
}
