use block::Block;
use celsium::block;
use celsium::block::TextSpan;
use celsium::compiletime_helper::CompileTimeHelper;
use celsium::typestack::TypeStack;
use celsium::CelsiumProgram;
use celsium::Scope;
use hime_redist::ast::AstNode;
use hime_redist::symbols::SemanticElementTrait;
use hime_redist::text::TextPosition;
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
pub mod formater;
use compiler::Compiler;
use compiler::CompileTimeError;

use crate::errors::common_error;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn wasm_print(s: &str);
    fn get_stumbrs_data() -> String;
}

#[derive(Debug, Clone)]
pub struct InterpreterReturns {
    pub testing_stack: Vec<celsium::vm::StackValue>,
    pub errors: Vec<CompileTimeError>,
}

pub fn interpret(
    path: String,
    print_ast: bool,
    print_bytecode: bool,
    static_only: bool,
    testing_stack: bool
) -> InterpreterReturns {
    let file_content = read_file(path.clone());
    let compile_helper = CompileTimeHelper::new(file_content.clone(), path.clone());
    let typestack: TypeStack = TypeStack::new();
    let mut compiler = Compiler {
        helper: compile_helper,
        is_wasm: false,
        typestack: typestack,
        functions: vec![],
        errors: vec![],
    };

    //send code to hime and get ast root
    let parse_res = hime::priede::parse_string(file_content.clone());
    //Add parser errors to the compiler struct
    parser::parser_errors(parse_res.errors.clone().errors, &mut compiler);

    //If there are compiler errors return at this moment.
    if compiler.errors.len() > 0 {
        for error in &compiler.errors {
            errors::print_error(error, &mut compiler.helper);
        }
        return InterpreterReturns { errors: compiler.errors, testing_stack: vec![] };
    }

    let ast = parse_res.get_ast();
    let root = ast.get_root();
    if print_ast {
        util::print_ast(root);
    }

    let mut main_block = Block::new(Scope { ast_id: root.id(), module_path: path.clone() });

    parse_ast::parse_ast(root, &mut compiler, &mut main_block);

    let mut i = 0;
    let mut jump_target_if_break_called = 0;
    while i < main_block.bytecode.len() {
        match &main_block.bytecode[i] {
            celsium::bytecode::OPTCODE::JumpIfFalse {
                steps,
                jump_target_line: _,
                jump_target_column: _,
                is_skipable,
            } => {
                if *is_skipable {
                    jump_target_if_break_called = i + steps;
                }
            }
            celsium::bytecode::OPTCODE::Break { span } => {
                if i > jump_target_if_break_called {
                    let span_c = span.clone();
                    common_error(
                        "Pārtraukums izmantots ārpus cikla",
                        Some(TextPosition { column: span_c.col_start, line: span_c.line }),
                        &mut compiler.helper
                    );
                    exit(1);
                }
                main_block.bytecode[i] = celsium::bytecode::OPTCODE::Jump {
                    steps: jump_target_if_break_called - i,
                };
            }
            celsium::bytecode::OPTCODE::Continue { span } => {
                if i > jump_target_if_break_called {
                    let span_c = span.clone();

                    common_error(
                        "Izlaišana izmantota ārpus cikla",
                        Some(TextPosition { column: span_c.col_start, line: span_c.line }),
                        &mut compiler.helper
                    );
                    exit(1);
                }
                main_block.bytecode[i] = celsium::bytecode::OPTCODE::Jump {
                    steps: jump_target_if_break_called - i - 1,
                };
            }
            _ => print!(""),
        }
        // println!("{} {:?}", i, main_block.bytecode[i]);
        i += 1;
    }

    if print_bytecode {
        let mut i = 0;
        while i < main_block.bytecode.len() {
            println!("{} {:?}", i, main_block.bytecode[i]);
            i += 1;
        }
    }
    if static_only {
        return InterpreterReturns { errors: compiler.errors, testing_stack: vec![] };
    }
    let mut celsium = CelsiumProgram::new(main_block, compiler.functions);
    let testing_stack_results = celsium.run_program();
    if testing_stack {
        println!("{:?}", testing_stack);
    }
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
    return InterpreterReturns {
        errors: compiler.errors,
        testing_stack: testing_stack_results,
    };
}

pub fn run_wasm(code: String) -> String {
    let parse_res = hime::priede::parse_string(code.clone());
    println!("{:?}", parse_res.errors.errors);
    let ast = parse_res.get_ast();
    let root = ast.get_root();

    let mut main_block = Block::new(Scope { ast_id: root.id(), module_path: "".to_string() });
    let compile_helper = CompileTimeHelper::new(code.clone(), "".to_string());
    let typestack: TypeStack = TypeStack::new();

    let mut compiler = Compiler {
        helper: compile_helper,
        is_wasm: false,
        functions: vec![],
        typestack: typestack,
        errors: vec![],
    };

    parse_ast::parse_ast(root, &mut compiler, &mut main_block);

    let mut celsium = CelsiumProgram::new(main_block, compiler.functions);
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
