use std::io::Stdin;

pub fn get_line(stdin: &Stdin) -> String {
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("Failed to read stdin");
    buf.trim().into()
}
