use wordler::{Recommends, State};

use wordler_cli::{AsDisplay, read_line};

fn main() {
    let mut state = State::default();
    let mut recommends = Recommends::new(&state);

    loop {
        let candidates = state.candidates();

        println!("{}", candidates.as_display());

        if candidates.len() <= 1 {
            break;
        }

        println!("{}", recommends.as_display());

        let guess = read_line("Guess", "guess");
        let hints = read_line("Hints", "hints");

        state.update(guess, hints);
        recommends.update(&state);

        println!();
    }
}
