use std::{env, process};

use wordler::{run, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|s| {
        eprintln!("Problem parsing arguments: {s}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
