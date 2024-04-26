use core::panic;

use crate::ast::*;
use celsium::{
    block::Block,
    module::{FuncArg, FunctionSignature, Module, VISIBILITY},
    bytecode::BINOP,
};
use hime_redist::{
    ast::{Ast, AstNode},
    symbols::SemanticElementTrait,
};

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn parse_ast(node: AstNode, block: &mut Block) {
    let title = node.get_symbol().to_string();
    if title == "func_call" {
        if node.children_count() > 1 {
            for arg in node.child(1) {
                parse_ast(arg, block);
            }
        }
        let func_name = node.child(0).get_value().unwrap();
        if func_name == "drukāt" {
            block.call_print_function(true);
        } else if func_name == "ievade"{
            block.input_function();
        } else {
            block.call_function(func_name);
        }
    } else if title == "return_st" {
        parse_ast(node.child(1), block);
        block.return_from_function();
    } else if title == "var_def" {
        println!("{}", node.child(0).get_symbol().to_string());
        if node.child(0).get_symbol().to_string() == "ARRAY" {
            for i in node.child(2).children() {
                parse_ast(i, block);
            }
            block.define_array(
                celsium::module::VISIBILITY::PRIVATE,
                node.child(1).get_value().unwrap().to_string(),
                node.child(2).children().len(),
            )
        } else {
            parse_ast(node.child(2), block);
            block.define_variable(
                match node.child(0).to_string().as_str() {
                    "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                    "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                    "TEXT" => celsium::BUILTIN_TYPES::STRING,

                    _ => panic!(),
                },
                celsium::module::VISIBILITY::PRIVATE,
                node.child(1).get_value().unwrap(),
            )
        }
    } else if title == "func_def" {
        let mut body = Block::new();
        let mut args: Vec<FuncArg> = vec![];

        if node.children_count() > 2 {
            //when the function takes arguments
            parse_ast(node.child(2), &mut body);
            for arg in node.child(1).children() {
                args.push(FuncArg {
                    name: arg.child(1).get_value().unwrap().to_string(),
                    arg_type: match arg.child(0).to_string().as_str() {
                        "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                        "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                        "TEXT" => celsium::BUILTIN_TYPES::MAGIC_INT,
                        _ => panic!(),
                    },
                })
            }
        } else {
            parse_ast(node.child(1), &mut body);
        }

        block.define_function(
            body,
            VISIBILITY::PUBLIC,
            FunctionSignature {
                name: node.child(0).get_value().unwrap().to_string(),
                return_type: celsium::module::FunctionReturnType::NONE,
                args: args,
            },
        )
    } else if title == "array" {
        parse_ast(node.child(1), block);
        block.load_from_array(node.child(0).get_value().unwrap());
    } else if title.starts_with("ID") {
        block.load_variable(node.get_value().unwrap());
    } else if title == "plus" || title == "string_plus" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(BINOP::ADD);
    } else if title == "minus" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(BINOP::SUBTRACT);
    } else if title == "reiz" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(BINOP::MULTIPLY);
    } else if title == "dal" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(BINOP::DIVIDE);
    } else if title == "atlik" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(BINOP::REMAINDER);
    } else if title == "if" {
        parse_ast(node.child(0), block);
        let mut if_block = Block::new();
        parse_ast(node.child(1), &mut if_block);
        if node.children_count() > 2 {
            let mut else_block = Block::new();
            parse_ast(node.child(3), &mut else_block);
            block.define_if_else_block(if_block, else_block)
        } else {
            block.define_if_block(if_block);
        }
    } else if title == "comp_s" {
        let sign = node.child(1).get_value().unwrap();
        parse_ast(node.child(0), block);
        parse_ast(node.child(2), block);
        match sign {
            "=" => block.binop(BINOP::EQ),
            ">" => block.binop(BINOP::LARGER_THAN),
            ">=" => block.binop(BINOP::LARGER_OR_EQ),
            "<" => block.binop(BINOP::LESS_THAN),
            "<=" => block.binop(BINOP::LESS_OR_EQ),
            "!=" => block.binop(BINOP::NOT_EQ),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        }
    } else if title == "un" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(BINOP::AND);
    } else if title == "vai" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(BINOP::OR);
    } else if title == "xvai" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(BINOP::XOR);
    } else if title == "block" {
        for i in node.children() {
            parse_ast(i, block);
        }
    } else if title == "s_loop" {
        let mut loop_block = Block::new();
        let mut loop_count_block = Block::new();

        parse_ast(node.child(1), &mut loop_block);
        parse_ast(node.child(0), &mut loop_count_block);
        block.define_simple_loop(loop_block, loop_count_block);
    } else if title == "w_loop" {
        let mut loop_block = Block::new();
        let mut conditional_block = Block::new();
        parse_ast(node.child(0), &mut conditional_block);
        parse_ast(node.child(1), &mut loop_block);
        block.define_while_loop(loop_block, conditional_block);
    } else if title == "id_asign" {
        let operator = node.child(1).get_value().unwrap();
        let var_name = node.child(0).get_value().unwrap();
        if operator == ":" {
            parse_ast(node.child(2), block);
        } else if operator == "+:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block);
            block.binop(BINOP::ADD);
        } else if operator == "-:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block);
            block.binop(BINOP::SUBTRACT);
        } else if operator == "*:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block);
            block.binop(BINOP::MULTIPLY);
        } else if operator == "/:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block);
            block.binop(BINOP::DIVIDE);
        } else if operator == "++" {
            block.load_variable(var_name);
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, "1");
            block.binop(BINOP::ADD);
        }
        block.assign_variable(var_name);
    } else if title == "NUMBER" {
        block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, node.get_value().unwrap());
    } else if title == "STRING" {
        block.load_const(
            celsium::BUILTIN_TYPES::STRING,
            rem_first_and_last(node.get_value().unwrap()),
        );
    }
}
