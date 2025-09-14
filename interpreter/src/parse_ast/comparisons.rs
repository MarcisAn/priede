use std::process::exit;

use celsium::{block::Block, bytecode::BINOP};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, Compiler };

use super::parse_ast;

pub fn comparisons(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "comp_s" {
        let sign = node.child(1).get_value().unwrap();
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(2), compiler, block);
        let checked_type = match sign {
            "=" => compiler.typestack.binop(BINOP::Eq),
            ">" => compiler.typestack.binop(BINOP::LargerThan),
            ">=" => compiler.typestack.binop(BINOP::LargerOrEq),
            "<" => compiler.typestack.binop(BINOP::LessThan),
            "<=" => compiler.typestack.binop(BINOP::LessOrEq),
            "!=" => compiler.typestack.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };
        if checked_type.is_none() {
            compiler.add_error(
                crate::compiler::CompileErrorType::MathTypes,
                node.child(1).get_position().unwrap().line,
                node.child(1).get_position().unwrap().column,
                node.child(1).get_span().unwrap().length
            );
            exit(0);
        }
        match sign {
            "=" => block.binop(BINOP::Eq),
            ">" => block.binop(BINOP::LargerThan),
            ">=" => block.binop(BINOP::LargerOrEq),
            "<" => block.binop(BINOP::LessThan),
            "<=" => block.binop(BINOP::LessOrEq),
            "!=" => block.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        }
    } else if title == "un" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        block.binop(BINOP::And);
        compiler.typestack.binop(BINOP::And);
    } else if title == "vai" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        block.binop(BINOP::Or);
        compiler.typestack.binop(BINOP::Or);
    } else if title == "xvai" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        block.binop(BINOP::Xor);
        compiler.typestack.binop(BINOP::Xor);
    }
}
