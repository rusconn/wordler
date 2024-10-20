mod candidates;
mod dict;
mod input;
mod letter;
mod recommends;
mod word;

use std::io;

use crate::{candidates::Candidates, input::Input, recommends::Recommends};

pub fn run() {
    let mut candidates = Candidates::default();
    let mut recommends = Recommends::default();
    let mut input = Input::default();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        if candidates.print() {
            return;
        }

        recommends.update(&candidates, &input);
        recommends.print();

        input.read(&stdin, &mut stdout);

        candidates.retain(&input);

        println!();
    }
}
