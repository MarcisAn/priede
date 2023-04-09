mod ast;
mod ast_parser;
mod hime;
mod interpreter;
use colored::*;
mod libloader;
mod state;

extern crate hime_redist;
extern crate priede_std;
use hime_redist::{
    ast::{Ast, AstNode},
    errors::{ParseError, ParseErrorDataTrait, ParseErrorUnexpectedToken},
    symbols::SemanticElementTrait,
    text::TextPosition,
};
use path_absolutize::Absolutize;
use relative_path::RelativePath;
use state::get_int_state;
//use hime_redist::symbols::SemanticElementTrait;
static mut AST_STR: String = String::new();
use std::{fs, path::Path};

use wasm_bindgen::prelude::*;

use crate::interpreter::print_error;

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

pub fn include_file(path: String, line: usize) {
    let (root, wasm) = get_int_state();

    if !wasm {
        let newpath = Path::new(&root).parent().unwrap().join(path.clone());
        let relative_path = RelativePath::new(&path);
        if newpath.exists() {
            interpret(false, newpath.into_os_string().into_string().unwrap(), wasm)
        } else {
            interpreter::print_error(
                line,
                format!(
                    "Nevar atrast failu adresē {}",
                    newpath
                        .absolutize()
                        .unwrap()
                        .as_os_str()
                        .to_os_string()
                        .to_str()
                        .unwrap()
                        .blue()
                ),
            );
        }
    } else {
        interpreter::print_error(line, "Nevar iekļaut failus pārlūkā".to_string());
    }
}

pub fn interpret(print_ast: bool, src_file: String, isWASM: bool) {
    if isWASM {
        state::set_int_state(true, "".to_string());
        let result = hime::priede::parse_string(&src_file);
        let ast = result.get_ast();
        ast_parser::parse_function(ast.get_root());
    } else {
        state::set_int_state(
            false,
            Path::new(&src_file)
                .absolutize()
                .unwrap()
                .as_os_str()
                .to_os_string()
                .into_string()
                .unwrap(),
        );
        let contents = fs::read_to_string(src_file).unwrap();
        let (sucess, result) = handle_parser_err(hime::priede::parse_string(&contents));
        if sucess {
            let ast = result.get_ast();
            if print_ast {
                print!("\n");
                format_ast(ast.get_root(), Vec::<bool>::new());
            }
            ast_parser::parse_function(ast.get_root());
        } else {
            print!("{}", "Programmas izpilde atlikta".red());
        }
    }
}

fn handle_parser_err(
    result: hime_redist::result::ParseResult,
) -> (bool, hime_redist::result::ParseResult) {
    let err_cnt = result.get_errors().get_count();
    let mut i = 0;
    while i < err_cnt {
        let error: ParseError = result.get_errors()[i].clone();
        let tokenerror;
        match error {
            ParseError::UnexpectedEndOfInput(_) => todo!(),
            ParseError::UnexpectedChar(_) => todo!(),
            ParseError::UnexpectedToken(val) => tokenerror = val,
            ParseError::IncorrectUTF16NoLowSurrogate(_) => todo!(),
            ParseError::IncorrectUTF16NoHighSurrogate(_) => todo!(),
        }
        print_error(
            tokenerror.get_position().line,
            tokenerror
                .get_message()
                .replace("Unexpected token", "Negaidīts vienums")
                .replace("; expected", ". sagaidīts"),
        );
        i += 1;
    }
    return (err_cnt == 0, result);
}

fn concat(input: String) {
    let text = input;
    unsafe {
        AST_STR = AST_STR.clone() + &text;
    }
}

fn format_ast<'a>(node: AstNode<'a>, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", "  ");
            concat("\x00".to_string());
            i += 1;
        }
        print!(" ");
        concat("\x00".to_string());
    }
    println!("{:}", node.to_string().blue());
    concat(node.to_string() + "\n");
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        format_ast(children.at(i), child_crossings);
        i += 1;
    }
}
