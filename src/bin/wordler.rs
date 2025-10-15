use std::{
    fmt,
    io::{self, Stdin, Stdout, Write},
};

use wordler::{Guess, Hints, Input};

fn main() {
    let mut input = Input::default();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let candidates = input.candidates();
        let recommends = input.recommends();

        println!("{candidates}");

        if candidates.len() <= 1 {
            break;
        }

        println!("{recommends}");

        let guess = read_guess(&stdin, &mut stdout);
        let hints = read_hints(&stdin, &mut stdout);
        input.update(guess, hints);

        println!();
    }
}

fn read_guess(stdin: &Stdin, stdout: &mut Stdout) -> Guess {
    read_line(stdin, stdout, "Guess", "guess")
}

fn read_hints(stdin: &Stdin, stdout: &mut Stdout) -> Hints {
    read_line(stdin, stdout, "Hints", "hints")
}

fn read_line<T>(stdin: &Stdin, stdout: &mut Stdout, label: &str, kind: &str) -> T
where
    T: for<'a> TryFrom<&'a str>,
    for<'a> <T as TryFrom<&'a str>>::Error: fmt::Display,
{
    loop {
        print!("{label}: ");
        stdout.flush().unwrap();

        let input = get_line(stdin);

        match T::try_from(&input) {
            Ok(t) => return t,
            Err(e) => eprintln!("Failed to read the {kind}: {e}"),
        }
    }
}

fn get_line(stdin: &Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().into()
}
