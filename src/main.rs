mod dict;
mod guess;
mod hints;

use std::{
    collections::HashSet,
    io::{self, Stdin},
};

use itertools::Itertools;

use wordler::{Candidates, LetterInfos, Recommends};

use self::dict::WORDS;
pub use self::guess::Guess;
use self::hints::Hints;

pub fn main() {
    let mut candidates = Candidates::from(&WORDS);
    let mut recommends = Recommends::from(&WORDS);
    let mut letter_infos = LetterInfos::new(5);

    let mut contains = HashSet::new();
    let mut not_contains = HashSet::new();
    let mut unuseds: HashSet<char> = HashSet::from_iter('A'..='Z');

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

        let guess = get_guess(&stdin);
        let hints = get_hints(&stdin);

        hints.apply(
            &guess,
            &mut letter_infos,
            &mut contains,
            &mut not_contains,
            &mut unuseds,
        );

        candidates.retain(&letter_infos, &contains, &not_contains);

        println!();
    }
}

fn get_guess(stdin: &Stdin) -> Guess {
    loop {
        eprint!("Guess: ");

        let input = get_line(stdin);

        match Guess::try_from(input.as_ref()) {
            Ok(guess) => return guess,
            Err(e) => eprintln!("Failed to read the guess: {e}"),
        }
    }
}

fn get_hints(stdin: &Stdin) -> Hints {
    loop {
        eprint!("Hints: ");

        let hints = get_line(stdin);

        match Hints::try_from(hints.as_ref()) {
            Ok(hints) => return hints,
            Err(e) => eprintln!("Failed to read the hints: {e}"),
        }
    }
}

fn get_line(stdin: &Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("Failed to read stdin");
    buf.trim().into()
}
