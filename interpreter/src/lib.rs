use block::Block;
use celsium::block;
use celsium::compiletime_helper::CompileTimeHelper;
use celsium::compiletime_helper::CompilerError;
use celsium::typestack::TypeStack;
use celsium::CelsiumProgram;
use celsium::Scope;
use hime_redist::ast::AstNode;
use hime_redist::symbols::SemanticElementTrait;
use util::stackvalue_to_json;
use std::panic;
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
        return InterpreterReturns { errors: compiler.errors, testing_stack: vec![] };
    }

    let ast = parse_res.get_ast();
    let root = ast.get_root();
    if print_ast {
        util::print_ast(root);
    }

    let mut main_block = Block::new(Scope { ast_id: root.id(), module_path: path.clone() });

    let mut block_ids: Vec<usize> = vec![];
    parse_block_ids(root, &mut block_ids);

    parse_ast::parse_ast(root, &mut compiler, &mut main_block);


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
