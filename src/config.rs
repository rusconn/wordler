use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub dict_path: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let dict_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the path to dictionary"),
        };

        Ok(Config { dict_path })
    }
}
