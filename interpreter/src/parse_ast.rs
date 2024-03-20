use crate::ast::*;
use celsium::{block::Block, module::Module};
use hime_redist::{
    ast::{Ast, AstNode},
    symbols::SemanticElementTrait,
};

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
    } else if title == "plus" {
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
    } else if title == "NUMBER" {
        block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, node.get_value().unwrap());
    }
}
