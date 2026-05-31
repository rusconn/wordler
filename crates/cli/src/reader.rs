use std::{
    io::{self, Write},
    str::FromStr,
};

use crate::CliError;

pub fn read_line<T>(label: &str) -> Option<T>
where
    T: FromStr,
    CliError: From<T::Err>,
{
    loop {
        print!("{label}: ");
        io::stdout().flush().unwrap();

        match get_line()?.parse() {
            Ok(t) => return Some(t),
            Err(e) => eprintln!("{}", CliError::from(e)),
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
