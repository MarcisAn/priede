mod ast;
mod ast_parser;
mod hime;
mod interpreter;
mod priede_std;
use colored::*;

extern crate hime_redist;
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};
//use hime_redist::symbols::SemanticElementTrait;
static mut AST_STR: String = String::new();
static mut IS_WASM: bool = false;
use std::fs;

use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn run_wasm(code: String) {
    interpret(true, code, true);
}

#[cfg(target_family = "wasm")]
pub fn console_log(out: &String) {
    log(&out);
}

pub fn main() {}
pub fn interpret(print_ast: bool, src_file: String, isWASM: bool) {
    unsafe {
        IS_WASM = isWASM;
    }
    if isWASM {
        let result = hime::priede::parse_string(&src_file);
        let ast = result.get_ast();
        ast_parser::parse_function(ast.get_root());
    } else {
        let contents = fs::read_to_string(src_file).unwrap();
        let result = hime::priede::parse_string(&contents);
        let ast = result.get_ast();
        if print_ast {
            format_ast(ast.get_root(), Vec::<bool>::new());
        }
        ast_parser::parse_function(ast.get_root());
    }
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
