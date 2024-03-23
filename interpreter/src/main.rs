use block::Block;
use celsium::block;
use celsium::module;
use celsium::CelsiumProgram;
use hime_redist::ast::AstNode;
use hime_redist::errors::ParseError;
use module::Module;
use std::fs::read;
use std::{fs, process};

fn main() {}

mod ast;
mod gen;
mod hime;
mod parse_ast;
mod util;

pub fn interpret(path: String) {
    let file_content = read_file(path);
    let parse_res = hime::priede::parse_string(file_content);
    println!("{:?}", parse_res.errors.errors);
    let ast = parse_res.get_ast();
    let root = ast.get_root();
    util::print_ast(root);

    let mut celsius = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsius);
    let mut main_block = Block::new();
    parse_ast::parse_ast(root.child(0), &mut main_block);
    //println!("{:?}", main_block.bytecode);
    main_module.add_main_block(main_block);
    celsius.add_module(&main_module);
    celsius.run_program();
}

pub fn run_wasm(code: String) {}

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
