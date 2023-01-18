mod ast;
mod ast_parser;
mod hime;
mod interpreter;
mod priede_std;

extern crate hime_redist;
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};
//use hime_redist::symbols::SemanticElementTrait;
static mut AST_STR: String = String::new();
static mut IS_WASM: bool = false;
use std::fs;
/*
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_wasm(code: String) {
    interpret(true, code, true);
}
 */
pub fn main() {}

pub fn interpret(print_ast: bool, src_file: String, isWASM: bool) {
    unsafe {
        IS_WASM = isWASM;
    }
    //let contents = fs::read_to_string(src_file).unwrap();
    ////print!("{:?}", contents);
    //let result = priede::parse_string(&contents);
    if isWASM {
        let result = hime::priede::parse_string(&src_file);
        let ast = result.get_ast();
        //if print_ast {
        //    format_ast(ast.get_root(), Vec::<bool>::new());
        //}
        ast_parser::parse_ast(ast.get_root());
    } else {
        let contents = fs::read_to_string(src_file).unwrap();
        //print!("{:?}", contents);
        let result = hime::priede::parse_string(&contents);
        let ast = result.get_ast();
        if print_ast {
            format_ast(ast.get_root(), Vec::<bool>::new());
        }

        ast_parser::parse_ast(ast.get_root());
    }
}
//fn write_to_file(input: String) -> io::Result<()> {
//    unsafe {
//        let data = input.as_bytes();
//
//        let mut pos = 0;
//        let mut buffer = File::open("foo.txt")?;
//
//        while pos < data.len() {
//            let bytes_written = buffer.write(&data[pos..])?;
//            pos += bytes_written;
//        }
//        Ok(())
//    }
//}

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
    println!("{:}", node);
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
