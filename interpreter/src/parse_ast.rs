use core::panic;

use crate::ast::*;
use celsium::{block::Block, module::Module, BINOP};
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
        for arg in node.child(1) {
            parse_ast(arg, block);
        }
        let func_name = node.child(0).get_value().unwrap();
        if func_name == "drukāt" {
            block.call_print_function(true);
        }
    } else if title == "var_def" {
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
    } else if title.starts_with("ID") {
        block.load_variable(node.get_value().unwrap());
    } else if title == "plus" || title == "string_plus" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(celsium::BINOP::ADD);
    } else if title == "minus" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(celsium::BINOP::SUBTRACT);
    } else if title == "reiz" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(celsium::BINOP::MULTIPLY);
    } else if title == "dal" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(celsium::BINOP::DIVIDE);
    } else if title == "atlik" {
        parse_ast(node.child(0), block);
        parse_ast(node.child(1), block);
        block.binop(celsium::BINOP::REMAINDER);
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
            "=" => block.binop(celsium::BINOP::EQ),
            ">" => block.binop(celsium::BINOP::LARGER_THAN),
            ">=" => block.binop(celsium::BINOP::LARGER_OR_EQ),
            "<" => block.binop(celsium::BINOP::LESS_THAN),
            "<=" => block.binop(celsium::BINOP::LESS_OR_EQ),
            "!=" => block.binop(celsium::BINOP::NOT_EQ),
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
        parse_ast(node.child(1), &mut loop_block);
        block.define_simple_loop(
            loop_block,
            node.child(0).get_value().unwrap().parse::<usize>().unwrap(),
        );
    } else if title == "NUMBER" {
        block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, node.get_value().unwrap());
    } else if title == "STRING" {
        block.load_const(
            celsium::BUILTIN_TYPES::STRING,
            rem_first_and_last(node.get_value().unwrap()),
        );
    }
}
