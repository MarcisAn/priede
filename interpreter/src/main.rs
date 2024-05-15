use block::Block;
use celsium::block;
use celsium::compile_time_checker::CompileTimeChecker;
use celsium::module;
use celsium::CelsiumProgram;
use errors::parser_error;
use hime_redist::errors::ParseErrorDataTrait;
use module::Module;
use std::panic;
use std::process::exit;
use std::{fs, process};
use wasm_bindgen::prelude::*;
extern crate stumbrs;
pub mod errors;

fn main() {}

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
    fn get_stumbrs_data() -> String;
}

pub fn get_stumbrs_data_wasm() -> String {
    //#[cfg(target_family = "wasm")]
    get_stumbrs_data()
}
fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn interpret(path: String, verbose: u8) {
    let file_content = read_file(path.clone());

    //send code to hime and get ast root
    let parse_res = hime::priede::parse_string(file_content.clone());
    for parse_err in parse_res.errors.clone().errors {
        let mut err = match parse_err {
            hime_redist::errors::ParseError::UnexpectedEndOfInput(_) => todo!(),
            hime_redist::errors::ParseError::UnexpectedChar(_) => todo!(),
            hime_redist::errors::ParseError::UnexpectedToken(a) => a,
            hime_redist::errors::ParseError::IncorrectUTF16NoLowSurrogate(_) => todo!(),
            hime_redist::errors::ParseError::IncorrectUTF16NoHighSurrogate(_) => todo!(),
        };
        let mut err_str = err.to_string();
        let expected_start = err.to_string().find("; expected").unwrap();
        let _ = err_str.split_off(expected_start);
        let unexpected_token = rem_first_and_last(&err_str.split_off(17)).to_string();
        parser_error(
            unexpected_token,
            &file_content,
            &path,
            err.get_position().line,
            err.get_position().column,
        );
    }
    if parse_res.errors.errors.len() > 0 {
        exit(0);
    }
    
    let ast = parse_res.get_ast();
    let root = ast.get_root();
    if verbose > 1 {
        util::print_ast(root);

    }

    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);
    let mut main_block = Block::new();

    parse_ast::parse_ast(
        root,
        &mut main_block,
        false,
        &mut CompileTimeChecker::new(file_content, path),
    );
    if verbose > 2 {

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
    let parse_res = hime::priede::parse_string(code.clone());
    println!("{:?}", parse_res.errors.errors);
    let ast = parse_res.get_ast();
    let root = ast.get_root();

    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);
    let mut main_block = Block::new();
    parse_ast::parse_ast(
        root,
        &mut main_block,
        true,
        &mut CompileTimeChecker::new(code, "".to_owned()),
    );
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
