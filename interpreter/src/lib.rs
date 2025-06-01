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
use hime_redist::utils;
use util::get_closest_node_location;
use util::get_furthest_node_location;
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

#[derive(Debug, Clone)]
pub struct FormatingContext<'a> {
    indentation_level: usize,
    indentation: &'a str,
}

pub fn format(code: String) -> String {
    let parse_res = hime::priede::parse_string(code.clone());

    if parse_res.errors.errors.len() > 0 {
        return code;
    }

    let ast = parse_res.get_ast();
    let root = ast.get_root();
    util::print_ast(root);

    let mut context = FormatingContext { indentation: "\t", indentation_level: 0 };

    return code_from_ast(root, &mut context).0.into();
}

fn code_from_ast<'a>(node: AstNode, context: &mut FormatingContext) -> (String, usize) {
    //return the formated string and number of lines
    let title = node.get_symbol().name;
    if title == "var_def" {
        return (
            format!(
                "{} {} : {}",
                node.child(0).get_value().unwrap(),
                node.child(1).get_value().unwrap(),
                code_from_ast(node.child(2), context).0
            ),
            1,
        );
    } else if title == "ID" {
        return (node.get_value().unwrap().to_string(), 0);
    } else if title == "NUMBER" {
        if node.children_count() > 0 {
            //id assign
            return (
                node.child(0).get_value().unwrap().to_string() + node.child(1).get_symbol().name,
                1,
            );
        } else {
            return (node.get_value().unwrap().to_string(), 0);
        }
    } else if title == "BOOL" {
        return (node.get_value().unwrap().to_string(), 0);
    } else if title == "STRING" {
        return (node.get_value().unwrap().to_string(), 0);
    } else if title == "return_st" {
        return (format!("atgriest {}", code_from_ast(node.child(0), context).0), 1);
    } else if title == "array" {
        let mut elements = "".to_string();
        for element in node.children() {
            elements += element.get_value().unwrap();
            if element != node.children().into_iter().last().unwrap() {
                elements += "; ";
            }
        }
        return (format!("[{}]", elements), 0);
    } else if title == "id_assign" {
        return (
            format!(
                "{} {} {}",
                code_from_ast(node.child(0), context).0,
                node.child(1).get_symbol().name,
                code_from_ast(node.child(2), context).0
            ),
            1,
        );
    } else if title == "func_call" {
        let mut arguments = String::new();
        for arg in node.child(1).children() {
            arguments += &code_from_ast(arg, context).0;
            if arg != node.child(1).children().into_iter().last().unwrap() {
                arguments += "; ";
            }
        }
        return (format!("{}({})", node.child(0).get_value().unwrap(), arguments), 1);
    } else if title == "comp_s" {
        return (
            format!(
                "{} {} {}",
                code_from_ast(node.child(0), context).0,
                node.child(1).get_symbol().name,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "plus" {
        return (
            format!(
                "{} + {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "minus" {
        return (
            format!(
                "{} - {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "reiz" {
        return (
            format!(
                "{} * {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "dal" {
        return (
            format!(
                "{} / {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "atlik" {
        return (
            format!(
                "{} % {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(2), context).0
            ),
            0,
        );
    } else if title == "un" {
        return (
            format!(
                "{} un {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(1), context).0
            ),
            0,
        );
    } else if title == "vai" {
        return (
            format!(
                "{} vai {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(1), context).0
            ),
            0,
        );
    } else if title == "xvai" {
        return (
            format!(
                "{} xvai {}",
                code_from_ast(node.child(0), context).0,
                code_from_ast(node.child(1), context).0
            ),
            0,
        );
    } else if title == "s_loop" {
        let condition = code_from_ast(node.child(0), context);
        let body = code_from_ast(node.child(1), context);
        return (format!("atkārtot ({}) {{\n{}\n}}", condition.0, body.0), 2 + condition.1 + body.1);
    } else if title == "w_loop" {
        let condition = code_from_ast(node.child(0), context);
        let body = code_from_ast(node.child(1), context);
        return (format!("kamēr ({}) {{\n{}\n}}", condition.0, body.0), 2 + condition.1 + body.1);
    } else if title == "block" {
        let mut block: String = "".into();
        let mut last_ending = 1;
        let mut is_first_child_node = true;
        let start = util::get_closest_node_location(node.child(0)).line;
        let mut end = 0;
        if node.parent().is_some() {
            context.indentation_level += 1;
        }
        for statement in node.children() {
            let statement_start = util::get_closest_node_location(statement).line;
            let parsed_statement = code_from_ast(statement, context);
            end = statement_start + parsed_statement.1 - 1;
            let mut empty_lines = statement_start - last_ending;
            last_ending = end;
            if is_first_child_node {
                empty_lines = 0;
            }
            is_first_child_node = false;
            block += &format!(
                "{}{}{}",
                "\n".repeat(empty_lines),
                context.indentation.repeat(context.indentation_level),
                parsed_statement.0
            );
        }
        if node.parent().is_some() {
            context.indentation_level -= 1;
        }
        let block_width = if end < start { 0 } else { end - start + 1 };
        return (block.to_string(), block_width);
    } else if title == "if" {
        let condition = code_from_ast(node.child(0), context);
        let body = code_from_ast(node.child(1), context);
        return (format!("ja ({}) {{\n{}\n{}}}", condition.0, body.0, context.indentation.repeat(context.indentation_level)), 2 + condition.1 + body.1);
    } else {
        ("".into(), 0)
    }
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
