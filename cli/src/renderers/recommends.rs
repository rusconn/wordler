use itertools::Itertools;

use wordler::Recommends;

pub fn render(recommends: &Recommends) -> String {
    if recommends.is_empty() {
        "Recommend: -".into()
    } else {
        format!("Recommend: [{}]", recommends.iter().take(5).join(","))
    }
}
