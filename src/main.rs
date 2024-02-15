use std::{
    collections::HashSet,
    io::{self, Stdin},
};

use itertools::Itertools;

use wordler::{
    candidates::Candidates, dict::WORDS, hint::Hint, letter_infos::LetterInfos,
    recommends::Recommends,
};

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

        let word = get_word(&stdin);
        let hints = get_hints(&stdin);

        for (i, (letter, hint)) in word.chars().zip(hints.iter()).enumerate() {
            match hint {
                Hint::NotExists => {
                    letter_infos.not(i, letter);
                    not_contains.insert(letter);
                }
                Hint::WrongSpot => {
                    letter_infos.not(i, letter);
                    contains.insert(letter);
                }
                Hint::CorrectSpot => {
                    letter_infos.correct(i, letter);
                    contains.insert(letter);
                }
            }

            unuseds.remove(&letter);
        }

        candidates.retain(&letter_infos, &contains, &not_contains);

        println!();
    }
}

fn get_word(stdin: &Stdin) -> String {
    loop {
        eprint!("Word: ");

        let input = get_line(stdin);

        if input.chars().count() != 5 {
            eprintln!("Word must be 5 letters");
            continue;
        }
        if !input.chars().all(|c| c.is_ascii_alphabetic()) {
            eprintln!("All letters must be ascii alphabetic");
            continue;
        }

        return input.to_ascii_uppercase();
    }
}

fn get_hints(stdin: &Stdin) -> Vec<Hint> {
    loop {
        eprint!("Hint: ");

        let hints = get_line(stdin);

        if hints.chars().count() != 5 {
            eprintln!("Hint must be 5 letters");
            continue;
        }

        match hints.chars().map(Hint::try_from).collect() {
            Ok(hints) => return hints,
            Err(e) => eprintln!("Failed to read the hint: {e}"),
        }
    }
}

fn get_line(stdin: &Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("Failed to read stdin");
    buf.trim().into()
}
