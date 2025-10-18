mod error;

use std::{
    io::{self, Write},
    str::FromStr,
};

use itertools::Itertools;

use wordler::{Candidates, Recommends, State};

use self::error::ParseError;

fn main() {
    let mut state = State::default();

    loop {
        let candidates = state.candidates();
        let recommends = state.recommends();

        println!("{}", show_candidates(candidates));

        if candidates.len() <= 1 {
            break;
        }

        println!("{}", show_recommends(recommends));

        let guess = read_line("Guess", "guess");
        let hints = read_line("Hints", "hints");
        state.update(guess, hints);

        println!();
    }
}

fn show_candidates(candidates: &Candidates) -> String {
    match candidates.len() {
        0 => "Woops, there are no more words".into(),
        1 => format!("Found: {}", candidates.first().unwrap()),
        n if n <= 50 => format!("Remaining: [{}]", candidates.iter().join(",")),
        n => format!("Remaining: Too many, didn't print: {n}"),
    }
}

fn show_recommends(recommends: &Recommends) -> String {
    if recommends.is_empty() {
        "Recommend: -".into()
    } else {
        format!("Recommend: [{}]", recommends.iter().take(5).join(","))
    }
}

fn read_line<T>(label: &str, kind: &str) -> T
where
    T: FromStr,
    ParseError: From<<T>::Err>,
{
    loop {
        print!("{label}: ");
        io::stdout().flush().unwrap();

        let input = get_line();

        match input.parse::<T>() {
            Ok(t) => return t,
            Err(e) => eprintln!("Failed to read the {kind}: {}", ParseError::from(e)),
        }
    }
}

fn get_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().into()
}
