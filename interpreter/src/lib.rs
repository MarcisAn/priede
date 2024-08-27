use block::Block;
use celsium::block;
use celsium::compiletime_helper::CompileTimeHelper;
use celsium::module;
use celsium::typestack;
use celsium::typestack::TypeStack;
use celsium::CelsiumProgram;
use celsium::Scope;
use compiler::CompileError;
use hime_redist::ast::AstNode;
use hime_redist::symbols::SemanticElementTrait;
use util::stackvalue_to_json;
use std::panic;
use std::process::exit;
use std::{ fs, process };
use wasm_bindgen::prelude::*;
pub mod errors;
mod hime;
mod parse_ast;
mod util;
mod compiler;
mod parser;
use compiler::Compiler;

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
#[derive(Debug, Clone)]
pub struct InterpreterReturns {
    testing_stack: Vec<celsium::vm::StackValue>,
    errors: Vec<CompileError>,
}

pub fn interpret(path: String, verbose: u8, static_only: bool) -> InterpreterReturns {
    let file_content = read_file(path.clone());
    let mut compile_helper = CompileTimeHelper::new(file_content.clone(), path.clone());
    let mut typestack: TypeStack = TypeStack::new();

    //send code to hime and get ast root
    let parse_res = hime::priede::parse_string(file_content.clone());
    let parser_errors = parser::parser_errors(parse_res.errors.clone().errors, &mut compile_helper);

    if parser_errors.len() > 0 {
        return InterpreterReturns { errors: parser_errors, testing_stack: vec![] };
    }

    let ast = parse_res.get_ast();
    let root = ast.get_root();
    if verbose > 1 {
        util::print_ast(root);
    }

    let mut main_block = Block::new(Scope { ast_id: root.id(), module_path: path.clone() });

    let mut block_ids: Vec<usize> = vec![];
    parse_block_ids(root, &mut block_ids);

    let mut compiler = Compiler {
        block: main_block,
        helper: compile_helper,
        is_wasm: false,
        errors: vec![],
        typestack: typestack,
        functions: vec![],
    };

    parse_ast::parse_ast(root, &mut compiler);

    if verbose > 2 {
        let mut i = 0;
        while i < compiler.block.bytecode.len() {
            println!("{} {:?}", i, compiler.block.bytecode[i]);
            i += 1;
        }
    }

    if static_only {
        return InterpreterReturns { errors: compiler.errors, testing_stack: vec![] };
    }
    let mut celsium = CelsiumProgram::new(compiler.block, compiler.functions);
    let testing_stack_results = celsium.run_program();
    let mut testing_stack_json: String = "[".to_string();
    let mut counter = 0;
    for value in testing_stack_results.clone() {
        testing_stack_json += &stackvalue_to_json(&value);
        counter += 1;
        if counter != testing_stack_results.len() {
            testing_stack_json += ",";
        }
    }
    testing_stack_json += "]";
    println!("{}", testing_stack_json);
    //return testing_stack_json;
    //println!("{:?}", testing_stack_results);
    return InterpreterReturns { errors: compiler.errors, testing_stack: testing_stack_results };
}

fn parse_block_ids(node: AstNode, block_ids: &mut Vec<usize>) {
    for child_node in node.children() {
        if node.get_symbol().name == "block" {
            //println!("{}", node.id());
        }
        parse_block_ids(child_node, block_ids);
    }
}

pub fn run_wasm(code: String) -> String {
    let parse_res = hime::priede::parse_string(code.clone());
    println!("{:?}", parse_res.errors.errors);
    let ast = parse_res.get_ast();
    let root = ast.get_root();

    let mut main_block = Block::new(Scope { ast_id: root.id(), module_path: "".to_string() });
    let compile_helper = CompileTimeHelper::new(code.clone(), "".to_string());
    let mut typestack: TypeStack = TypeStack::new();

    let mut compiler = Compiler {
        block: main_block,
        helper: compile_helper,
        is_wasm: false,
        errors: vec![],
        functions: vec![],
        typestack: typestack,
    };

    parse_ast::parse_ast(root, &mut compiler);

    let mut celsium = CelsiumProgram::new(compiler.block, compiler.functions);
    let testing_stack_results = celsium.run_program();
    let mut testing_stack_json: String = "[".to_string();
    for value in testing_stack_results {
        testing_stack_json += &stackvalue_to_json(&value);
    }
    return testing_stack_json;
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
