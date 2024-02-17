mod candidates;
mod dict;
mod input;
mod letter;
mod letter_infos;
mod recommends;
mod word;

use std::{collections::HashSet, io};

use self::{
    candidates::Candidates, dict::WORDS, input::Input, letter::Letter, letter_infos::LetterInfos,
    recommends::Recommends, word::Word,
};

pub fn run() {
    let mut candidates = Candidates::unsafe_from(&WORDS);
    let mut recommends = Recommends::unsafe_from(&WORDS);
    let mut letter_infos = LetterInfos::new(5);

    let mut contains = HashSet::new();
    let mut not_contains = HashSet::new();
    let mut unuseds = HashSet::from_iter(('A'..='Z').map(Letter::unsafe_from));

    let stdin = io::stdin();

    loop {
        if candidates.print() {
            return;
        }

        recommends.update(&candidates, &unuseds);
        recommends.print();

        let input = Input::read(&stdin);

        input.apply(
            &mut letter_infos,
            &mut contains,
            &mut not_contains,
            &mut unuseds,
        );

        candidates.retain(&letter_infos, &contains, &not_contains);

        println!();
    }
}
