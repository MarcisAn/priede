use std::io::{self, BufRead, Write};

#[cfg(target_family = "wasm")]
use crate::console_log;

fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}

pub fn print(i: String) -> String {
    #[cfg(target_family = "wasm")]
    console_log(&i);
    print!("{}", i);
    i
}
pub fn printnl(i: String) -> String {
    #[cfg(target_family = "wasm")]
    console_log(&i);
    print!("\n{}", i);
    i
}
pub fn ievade(i: String) -> String {
    let user_input = input(&i).unwrap();
    return user_input;
}
