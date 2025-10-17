use std::{
    io::{self, Stdin, Stdout, Write},
    str::FromStr,
};

use itertools::Itertools;

use wordler::{
    Candidates, Guess, Hints, ParseGuessError, ParseHintError, ParseHintsError, ParseLetterError,
    Recommends, State,
};

fn main() {
    let mut state = State::default();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let candidates = state.candidates();
        let recommends = state.recommends();

        println!("{}", show_candidates(candidates));

        if candidates.len() <= 1 {
            break;
        }

        println!("{}", show_recommends(recommends));

        let guess = read_guess(&stdin, &mut stdout);
        let hints = read_hints(&stdin, &mut stdout);
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

fn read_guess(stdin: &Stdin, stdout: &mut Stdout) -> Guess {
    read_line(stdin, stdout, "Guess", "guess", show_parse_guess_error)
}

fn read_hints(stdin: &Stdin, stdout: &mut Stdout) -> Hints {
    read_line(stdin, stdout, "Hints", "hints", show_parse_hints_error)
}

fn read_line<T: FromStr>(
    stdin: &Stdin,
    stdout: &mut Stdout,
    label: &str,
    kind: &str,
    show_parse_error: impl Fn(<T as FromStr>::Err) -> String,
) -> T {
    loop {
        print!("{label}: ");
        stdout.flush().unwrap();

        let input = get_line(stdin);

        match input.parse::<T>() {
            Ok(t) => return t,
            Err(e) => eprintln!("Failed to read the {kind}: {}", show_parse_error(e)),
        }
    }
}

fn get_line(stdin: &Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().into()
}

fn show_parse_guess_error(e: ParseGuessError) -> String {
    match e {
        ParseGuessError::InvalidLength => "Guess must be 5 letters".into(),
        ParseGuessError::UnknownWord => "Unknown word".into(),
        ParseGuessError::InvalidLetter(e) => show_parse_letter_error(e),
    }
}

fn show_parse_letter_error(e: ParseLetterError) -> String {
    match e {
        ParseLetterError::NonAlphabeticalLetter(c) => {
            format!("Non alphabetical letter: `{c}`")
        }
    }
}

fn show_parse_hints_error(e: ParseHintsError) -> String {
    match e {
        ParseHintsError::InvalidLength => "Hints must be 5 letters".into(),
        ParseHintsError::InvalidHint(e) => show_parse_hint_error(e),
    }
}

fn show_parse_hint_error(e: ParseHintError) -> String {
    match e {
        ParseHintError::InvalidHint(c) => format!("Invalid hint: `{c}`"),
    }
}
