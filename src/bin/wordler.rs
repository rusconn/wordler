use std::io::{self, Stdin, Stdout, Write};

use itertools::Itertools;

use wordler::{
    Candidates, Input, Recommends,
    guess::{self, Guess},
    hint,
    hints::{self, Hints},
    letter,
};

fn main() {
    let mut input = Input::default();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let candidates = input.candidates();
        let recommends = input.recommends();

        println!("{}", show_candidates(candidates));

        if candidates.len() <= 1 {
            break;
        }

        println!("{}", show_recommends(recommends));

        let guess = read_guess(&stdin, &mut stdout);
        let hints = read_hints(&stdin, &mut stdout);
        input.update(guess, hints);

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
    read_line(stdin, stdout, "Guess", "guess", show_guess_conversion_error)
}

fn read_hints(stdin: &Stdin, stdout: &mut Stdout) -> Hints {
    read_line(stdin, stdout, "Hints", "hints", show_hints_conversion_error)
}

fn read_line<T>(
    stdin: &Stdin,
    stdout: &mut Stdout,
    label: &str,
    kind: &str,
    show_conversion_error: impl Fn(<T as TryFrom<&str>>::Error) -> String,
) -> T
where
    T: for<'a> TryFrom<&'a str>,
{
    loop {
        print!("{label}: ");
        stdout.flush().unwrap();

        let input = get_line(stdin);

        match T::try_from(&input) {
            Ok(t) => return t,
            Err(e) => eprintln!("Failed to read the {kind}: {}", show_conversion_error(e)),
        }
    }
}

fn get_line(stdin: &Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().into()
}

fn show_guess_conversion_error(e: guess::ParseError) -> String {
    match e {
        guess::ParseError::InvalidLength => "Guess must be 5 letters".into(),
        guess::ParseError::UnknownWord => "Unknown word".into(),
        guess::ParseError::InvalidLetter(e) => show_letter_conversion_error(e),
    }
}

fn show_letter_conversion_error(e: letter::ParseError) -> String {
    match e {
        letter::ParseError::NonAlphabeticalLetter(c) => {
            format!("Non alphabetical letter: `{c}`")
        }
    }
}

fn show_hints_conversion_error(e: hints::ParseError) -> String {
    match e {
        hints::ParseError::InvalidLength => "Hints must be 5 letters".into(),
        hints::ParseError::InvalidHint(e) => show_hint_conversion_error(e),
    }
}

fn show_hint_conversion_error(e: hint::ParseError) -> String {
    match e {
        hint::ParseError::InvalidHint(c) => format!("Invalid hint: `{c}`"),
    }
}
