use crate::ast::*;
use celsium::{block::Block, module::Module, BINOP};
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
    } else if title == "NUMBER" {
        block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, node.get_value().unwrap());
    }
}
