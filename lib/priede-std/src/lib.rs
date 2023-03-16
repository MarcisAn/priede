use std::num;
use std::{
    io::{self, BufRead, Write},
    ops::{Mul, Shr},
};
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

pub fn kapinat(base: i128, pow: i128) -> i128 {
    base.pow(pow as u32)
}

pub fn kvadratsakne(a: i128) -> i128 {
    let b = a as f64;
    b.sqrt() as i128
}
pub fn comp_int(a: i128, b: i128) -> bool {
    return a == b;
}
pub fn comp_str(a: String, b: String) -> bool {
    return a == b;
}
pub fn comp_bool(a: bool, b: bool) -> bool {
    return a == b;
}
