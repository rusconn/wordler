use wordler::State;

use wordler_cli::{read_line, render_candidates, render_recommends};

fn main() {
    let mut state = State::default();

    loop {
        let candidates = state.candidates();
        let recommends = state.recommends();

        println!("{}", render_candidates(candidates));

        if candidates.len() <= 1 {
            break;
        }

        println!("{}", render_recommends(recommends));

        let guess = read_line("Guess", "guess");
        let hints = read_line("Hints", "hints");
        state.update(guess, hints);

        println!();
    }
}
