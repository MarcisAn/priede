use block::Block;
use celsium::block;
use celsium::compiletime_helper::CompileTimeHelper;
use celsium::module;
use celsium::CelsiumProgram;
use celsium::Scope;
use hime_redist::ast::AstNode;
use hime_redist::errors::ParseError;
use hime_redist::errors::ParseErrorDataTrait;
use hime_redist::errors::ParseErrorUnexpectedChar;
use hime_redist::errors::ParseErrorUnexpectedToken;
use hime_redist::symbols::SemanticElementTrait;
use module::Module;
use std::panic;
use std::process::exit;
use std::{ fs, process };
use wasm_bindgen::prelude::*;
pub mod errors;
mod data_format;
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

fn unexpected_token_error(err: ParseErrorUnexpectedToken, compilehelper: &mut CompileTimeHelper) {
    let mut err_str = err.to_string();
    let expected_start = err.to_string().find("; expected").unwrap();
    let _ = err_str.split_off(expected_start);
    let unexpected_token = rem_first_and_last(&err_str.split_off(17)).to_string();
    errors::parser_error(unexpected_token, err.get_position(), compilehelper);
}
fn unexpected_char_error(err: ParseErrorUnexpectedChar, compilehelper: &mut CompileTimeHelper) {
    let mut err_str = err.to_string();
    let expected_start = err.to_string().find("'").unwrap();
    let mut split = err_str.split_off(expected_start);
    let _ = split.split_off(split.len() - 8);
    let unexpected_token = split.split_off(1);

    errors::parser_error(unexpected_token, err.get_position(), compilehelper);
}

pub fn interpret(path: String, verbose: u8) {
    let file_content = read_file(path.clone());

    let mut compile_helper = CompileTimeHelper::new(file_content.clone(), path.clone());

    //send code to hime and get ast root
    let parse_res = hime::priede::parse_string(file_content.clone());
    for parse_err in parse_res.errors.clone().errors {
        match parse_err {
            hime_redist::errors::ParseError::UnexpectedEndOfInput(_) => todo!(),
            hime_redist::errors::ParseError::UnexpectedChar(err) =>
                unexpected_char_error(err, &mut compile_helper),
            hime_redist::errors::ParseError::UnexpectedToken(err) =>
                unexpected_token_error(err, &mut compile_helper),
            hime_redist::errors::ParseError::IncorrectUTF16NoLowSurrogate(_) => todo!(),
            hime_redist::errors::ParseError::IncorrectUTF16NoHighSurrogate(_) => todo!(),
        }
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
    let mut main_block = Block::new(Scope { ast_id: root.id(), module_path: path });

    let mut block_ids: Vec<usize> = vec![];
    parse_block_ids(root, &mut block_ids);

    parse_ast::parse_ast(root, &mut main_block, false, &mut compile_helper);

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

fn parse_block_ids(node: AstNode, block_ids: &mut Vec<usize>) {
    for child_node in node.children() {
        if node.get_symbol().name == "block" {
            //println!("{}", node.id());
        }
        parse_block_ids(child_node, block_ids);
    }
}

pub fn run_wasm(code: String) {
    let parse_res = hime::priede::parse_string(code.clone());
    println!("{:?}", parse_res.errors.errors);
    let ast = parse_res.get_ast();
    let root = ast.get_root();

    let mut celsium = CelsiumProgram::new();
    let mut main_module = Module::new("main", &mut celsium);
    let mut main_block = Block::new(Scope { ast_id: root.id(), module_path: "".to_string() });
    parse_ast::parse_ast(
        root,
        &mut main_block,
        true,
        &mut CompileTimeHelper::new(code, "".to_owned())
    );
    main_module.add_main_block(main_block.clone());
    celsium.add_module(&main_module);

    celsium.run_program();
}

pub fn read_file(path: String) -> String {
    let file_read = fs::read_to_string(&path);
    if file_read.is_err() {
        println!("{}", file_read.err().unwrap());
        println!("Neizdevās nolasīt failu {} \nPārlicinies, ka faila adrese ir pareiza!", path);
        process::exit(1);
    }
    file_read.unwrap()
}

pub fn parser_errors(errors: Vec<ParseError>, compile_helper: &mut CompileTimeHelper) {
    for parse_err in errors.clone() {
        match parse_err {
            hime_redist::errors::ParseError::UnexpectedEndOfInput(_) => todo!(),
            hime_redist::errors::ParseError::UnexpectedChar(err) =>
                unexpected_char_error(err, compile_helper),
            hime_redist::errors::ParseError::UnexpectedToken(err) =>
                unexpected_token_error(err, compile_helper),
            hime_redist::errors::ParseError::IncorrectUTF16NoLowSurrogate(_) => todo!(),
            hime_redist::errors::ParseError::IncorrectUTF16NoHighSurrogate(_) => todo!(),
        }
    }
    if errors.len() > 0 {
        exit(0);
    }
}
