mod executor;
mod generator;
use executor::executor::Executor as bytecode_executor;

extern crate hime_redist;
use generator::generator::generate_bytcode;

use std::{fs, process};

use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn input(prompt: &str) -> String;
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn run_wasm(code: String) {
    interpret(true, code, true);
}

#[cfg(target_family = "wasm")]
fn input_fn(prompt: &str) {
    input("cav");
}

fn main() {}

pub fn interpret(print_ast: bool, src_file: String, isWASM: bool) {
    let file_read = fs::read_to_string(src_file.clone());
    if file_read.is_err() {
        println!(
            "Neizdevās nolasīt failu {} \nPārlicinies, ka faila adrese ir pareiza!",
            src_file
        );
        process::exit(1);
    }
    let contents = file_read.unwrap();
    let bytecode = generate_bytcode(contents.clone(), print_ast).unwrap();
    //println!("{:?}", bytecode);
    for (index, i) in bytecode.clone().into_iter().enumerate() {
        println!("[{}] {:?}", index, i.typ);
    }
    let mut executor = bytecode_executor::new();
    executor::executor::Executor::execute_bytecode(&mut executor, bytecode);
}
