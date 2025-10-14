use std::io::{self, Stdin, Stdout, Write};

use wordler::{Candidates, Guess, Hints, Input, Recommends};

fn main() {
    let mut candidates = Candidates::default();
    let mut recommends = Recommends::default();
    let mut input = Input::default();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        println!("{candidates}");

        if candidates.len() <= 1 {
            break;
        }

        recommends.update(&candidates, &input);

        println!("{recommends}");

        let guess = read_guess(&stdin, &mut stdout);
        let hints = read_hints(&stdin, &mut stdout);
        input.update(guess, hints);

        candidates.retain(&input);

        println!();
    }
}

fn read_guess(stdin: &Stdin, stdout: &mut Stdout) -> Guess {
    loop {
        print!("Guess: ");
        stdout.flush().unwrap();

        let guess = get_line(stdin);

        match Guess::try_from(guess.as_ref()) {
            Ok(guess) => return guess,
            Err(e) => eprintln!("Failed to read the guess: {e}"),
        }
    }
}

fn read_hints(stdin: &Stdin, stdout: &mut Stdout) -> Hints {
    loop {
        print!("Hints: ");
        stdout.flush().unwrap();

        let hints = get_line(stdin);

        match Hints::try_from(hints.as_ref()) {
            Ok(hints) => return hints,
            Err(e) => eprintln!("Failed to read the hints: {e}"),
        }
    }
}

fn get_line(stdin: &Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().into()
}
