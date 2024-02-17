mod candidates;
mod dict;
mod input;
mod letter_infos;
mod recommends;
mod word;

use std::{collections::HashSet, io};

use itertools::Itertools;

use self::{
    candidates::Candidates, dict::WORDS, input::Input, letter_infos::LetterInfos,
    recommends::Recommends, word::Word,
};

pub fn run() {
    let mut candidates = Candidates::from(&WORDS);
    let mut recommends = Recommends::from(&WORDS);
    let mut letter_infos = LetterInfos::new(5);

    let mut contains = HashSet::new();
    let mut not_contains = HashSet::new();
    let mut unuseds = HashSet::from_iter('A'..='Z');

    let stdin = io::stdin();

    loop {
        match candidates.len() {
            0 => return println!("Woops, there are no more words"),
            1 => return println!("Found: {}", candidates[0]),
            n if n <= 50 => println!("Remaining: [{}]", candidates.iter().join(",")),
            n => println!("Remaining: Too many, didn't print: {n}"),
        }

        recommends.update(&candidates, &unuseds);

        if recommends.is_empty() {
            println!("Recommend: -");
        } else {
            println!("Recommend: [{}]", recommends.take(5).join(","));
        }

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
