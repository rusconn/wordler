use std::{
    io::{self, Write},
    str::FromStr,
};

use crate::ParseError;

pub fn read_line<T>(label: &str, kind: &str) -> Option<T>
where
    T: FromStr,
    ParseError: From<T::Err>,
{
    loop {
        print!("{label}: ");
        io::stdout().flush().unwrap();

        match get_line()?.parse() {
            Ok(t) => return Some(t),
            Err(e) => eprintln!("Failed to read the {kind}: {}", ParseError::from(e)),
        }
    }
}

fn get_line() -> Option<String> {
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).unwrap() == 0 {
        return None;
    }
    Some(buf.trim().into())
}
