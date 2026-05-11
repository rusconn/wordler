use std::{
    io::{self, Write},
    str::FromStr,
};

use crate::ParseError;

pub fn read_line<T>(label: &str, kind: &str) -> T
where
    T: FromStr,
    ParseError: From<T::Err>,
{
    loop {
        print!("{label}: ");
        io::stdout().flush().unwrap();

        match get_line().parse() {
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
