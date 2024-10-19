mod candidates;
mod dict;
mod input;
mod letter;
mod letter_infos;
mod recommends;
mod word;

use std::io;

use crate::{
    candidates::Candidates, input::Input, letter_infos::LetterInfos, recommends::Recommends,
};

pub fn run() {
    let mut candidates = Candidates::default();
    let mut recommends = Recommends::default();
    let mut letter_infos = LetterInfos::default();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        if candidates.print() {
            return;
        }

        recommends.update(&candidates, &letter_infos);
        recommends.print();

        let input = Input::read(&stdin, &mut stdout);

        letter_infos.apply(input);

        candidates.retain(&letter_infos);

        println!();
    }
}
