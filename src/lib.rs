mod candidates;
mod dict;
mod excludes;
mod includes;
mod input;
mod letter;
mod letter_infos;
mod recommends;
mod veileds;
mod word;

use std::io;

use self::{
    candidates::Candidates, dict::WORDS, excludes::Excludes, includes::Includes, input::Input,
    letter::Letter, letter_infos::LetterInfos, recommends::Recommends, veileds::Veileds,
    word::Word,
};

pub fn run() {
    let mut candidates = Candidates::unsafe_from(WORDS);
    let mut recommends = Recommends::unsafe_from(WORDS);
    let mut veileds = Veileds::unsafe_from('A'..='Z');

    let mut letter_infos = LetterInfos::default();
    let mut includes = Includes::default();
    let mut excludes = Excludes::default();

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
