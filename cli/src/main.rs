use wordler::{Recommends, State};

use wordler_cli::{AsDisplay, CliError, read_line};

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

        if !interact_and_update(&mut state) {
            break;
        }

        recommends.update(&state);

        println!();
    }
}

fn interact_and_update(state: &mut State) -> bool {
    let Some(guess) = read_line("Guess") else {
        return false;
    };

    loop {
        let Some(hints) = read_line("Hints") else {
            return false;
        };

        match state.update(&guess, &hints) {
            Ok(_) => return true,
            Err(e) => {
                println!("{}", CliError::from(e));
            }
        }
    }
}
