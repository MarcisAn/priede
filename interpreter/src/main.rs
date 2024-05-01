use block::Block;
use celsium::block;
use celsium::module;
use celsium::CelsiumProgram;
use module::Module;
use std::panic;
use std::{fs, process};
use wasm_bindgen::prelude::*;
extern crate stumbrs;
use stumbrs::load_stumbrs_data;

fn main() {
}

mod ast;
mod hime;
mod parse_ast;
mod util;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
}

pub fn interpret(path: String, verbose: u8) {
    let file_content = read_file(path);

    //send code to hime and get ast root
    let parse_res = hime::priede::parse_string(file_content);
    println!("{:?}", parse_res.errors.errors);
    let ast = parse_res.get_ast();
    let root = ast.get_root();

    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);
    let mut main_block = Block::new();

    parse_ast::parse_ast(root, &mut main_block);

    if verbose >= 1 {
        util::print_ast(root);
        let mut i = 0;
        while i < main_block.bytecode.len() {
            println!("{} {:?}", i, main_block.bytecode[i]);
            i += 1;
        }
    }

    main_module.add_main_block(main_block);
    celsium.add_module(&main_module);
    celsium.run_program();
}

pub fn run_wasm(code: String) {
    let parse_res = hime::priede::parse_string(code);
    println!("{:?}", parse_res.errors.errors);
    let ast = parse_res.get_ast();
    let root = ast.get_root();

    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);
    let mut main_block = Block::new();
    parse_ast::parse_ast(root, &mut main_block);
    main_module.add_main_block(main_block.clone());
    celsium.add_module(&main_module);

    celsium.run_program();
}

fn read_file(path: String) -> String {
    let file_read = fs::read_to_string(&path);
    if file_read.is_err() {
        println!(
            "Neizdevās nolasīt failu {} \nPārlicinies, ka faila adrese ir pareiza!",
            path
        );
        process::exit(1);
    }
    file_read.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
        teksts a : \"Sveika, pasaule!\" \
        \
        drukāt(a)";

        let parse_res = hime::priede::parse_string(input.to_owned());
        println!("{:?}", parse_res.errors.errors);
        let ast = parse_res.get_ast();
        let root = ast.get_root();

        let result = panic::catch_unwind(|| {
            let mut celsium = CelsiumProgram::new();
            let mut main_module = Module::new("main", &mut celsium);
            let mut main_block = Block::new();
            parse_ast::parse_ast(root.child(0), &mut main_block);
            main_module.add_main_block(main_block);
            celsium.add_module(&main_module);

            celsium.run_program();
        });
    }
}
