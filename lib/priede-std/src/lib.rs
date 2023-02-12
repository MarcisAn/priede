use std::io::{self, BufRead, Write};
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[cfg(target_family = "wasm")]
pub fn console_log(out: &String) {
    log(&out);
}
#[cfg(target_family = "wasm")]
pub fn console_log_ln(out: &String) {
    alert(&out);
}

fn input(prompt: &str) -> io::Result<String> {
    print(prompt.to_string());
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
    alert(&i);
    print!("\n{}", i);
    i
}
pub fn ievade(i: String) -> String {
    let user_input = input(&i).unwrap();
    return user_input;
}
