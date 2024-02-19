mod candidates;
mod dict;
mod input;
mod letter;
mod letter_infos;
mod letter_set;
mod recommends;
mod word;

use std::io;

use self::{
    candidates::Candidates,
    dict::WORDS,
    input::Input,
    letter::Letter,
    letter_infos::LetterInfos,
    letter_set::{Excludes, Includes, NotLetters, Veileds, WordLetters},
    recommends::Recommends,
    word::Word,
};

pub fn run() {
    let mut candidates = Candidates::default();
    let mut recommends = Recommends::default();
    let mut letter_infos = LetterInfos::default();

    let mut includes = Includes::default();
    let mut excludes = Excludes::default();
    let mut veileds = Veileds::default();

    let stdin = io::stdin();

    loop {
        if candidates.print() {
            return;
        }

        recommends.update(&candidates, &veileds);
        recommends.print();

        let input = Input::read(&stdin);

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
