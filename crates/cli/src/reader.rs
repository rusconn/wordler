use std::{
    io::{self, Write},
    str::FromStr,
};

use crate::AsDisplay;

pub fn read_line<T>(label: &str) -> Option<T>
where
    T: FromStr,
    for<'a> T::Err: AsDisplay<'a>,
{
    loop {
        print!("{label}: ");
        io::stdout().flush().unwrap();

        match get_line()?.parse() {
            Ok(t) => return Some(t),
            Err(e) => eprintln!("{}", e.as_display()),
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
