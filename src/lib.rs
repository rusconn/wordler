mod candidates;
mod dict;
mod input;
mod letter;
mod letter_infos;
mod letter_set;
mod recommends;
mod word;

use std::io;

use crate::{
    candidates::Candidates,
    input::Input,
    letter_infos::LetterInfos,
    letter_set::{Excludes, Includes, Veileds},
    recommends::Recommends,
};

pub fn run() {
    let mut candidates = Candidates::default();
    let mut recommends = Recommends::default();
    let mut letter_infos = LetterInfos::default();

    let mut includes = Includes::default();
    let mut excludes = Excludes::default();
    let mut veileds = Veileds::default();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        if candidates.print() {
            return;
        }

        recommends.update(&candidates, &veileds);
        recommends.print();

        let input = Input::read(&stdin, &mut stdout);

        input.apply(
            &mut letter_infos,
            &mut includes,
            &mut excludes,
            &mut veileds,
        );

        candidates.retain(&letter_infos, &includes, &excludes);

        println!();
    }
}
