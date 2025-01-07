use std::{io, io::Write};

pub fn get_prompt(str: &str) -> String {
    print!("{}", str);
    io::stdout().flush().expect("failed to flush");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    return input.trim().to_string()
}

pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}
