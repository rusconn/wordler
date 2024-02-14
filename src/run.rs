use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    io::{self, Stdin},
};

use itertools::Itertools;
use regex::Regex;

use crate::{char_infos::CharInfos, config::Config, hint::Hint, io_util::get_line};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dict_content = fs::read_to_string(config.dict_path)?;

    let dict = dict_content
        .split_ascii_whitespace()
        .filter(|s| s.len() == 5)
        .map(|line| line.to_ascii_uppercase())
        .unique()
        .collect();

    wordle(dict);

    Ok(())
}

fn wordle(mut candidates: Vec<String>) {
    let mut recommend = candidates.clone();
    let mut char_infos = CharInfos::new(5);
    let mut contains = HashSet::new();
    let mut not_contains = HashSet::new();
    let mut unuseds: HashSet<char> = HashSet::from_iter('A'..='Z');
    let stdin = io::stdin();

    loop {
        let word = get_word(&stdin);
        let hints = get_hints(&stdin);

        for (i, (alpha, hint)) in word.chars().zip(hints.iter()).enumerate() {
            match hint {
                Hint::NotExists => {
                    char_infos.not(i, alpha);
                    not_contains.insert(alpha);
                }
                Hint::WrongSpot => {
                    char_infos.not(i, alpha);
                    contains.insert(alpha);
                }
                Hint::CorrectSpot => {
                    char_infos.correct(i, alpha);
                    contains.insert(alpha);
                }
            }

            unuseds.remove(&alpha);
        }

        let regex = Regex::new(&char_infos.as_regex())
            .unwrap_or_else(|e| panic!("Failed to create Regex: {e}"));

        candidates.retain(|word| {
            let word_chars = HashSet::from_iter(word.chars());
            regex.is_match(word)
                && contains.is_subset(&word_chars)
                && not_contains.is_disjoint(&word_chars)
        });

        match candidates.len() {
            0 => return println!("Woops, there are no more words"),
            1 => return println!("Found: {}", candidates[0]),
            n if n <= 50 => println!("Remaining: [{}]", &candidates.join(",")),
            n => println!("Remaining: Too many, didn't print: {}", n),
        }

        let mut histogram = HashMap::new();

        for chars in candidates.iter().map(|word| word.chars().unique()) {
            for ch in chars {
                if unuseds.contains(&ch) {
                    *histogram.entry(ch).or_insert(0) += 1;
                }
            }
        }

        recommend.sort_unstable_by_key(|word| {
            Reverse(
                word.chars()
                    .unique()
                    .map(|c| histogram.get(&c).unwrap_or(&0))
                    .sum::<i32>(),
            )
        });

        println!("Recommend: [{}]\n", &recommend[..5].join(","));
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
