use std::process::exit;

use celsium::bytecode::BINOP;
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, Compiler };

use super::parse_ast;

pub fn comparisons(node: AstNode, title: &str, compiler: &mut Compiler) {
    if title == "comp_s" {
        let sign = node.child(1).get_value().unwrap();
        parse_ast(node.child(0), compiler);
        parse_ast(node.child(2), compiler);
        let checked_type = match sign {
            "=" => compiler.helper.binop(BINOP::Eq),
            ">" => compiler.helper.binop(BINOP::LargerThan),
            ">=" => compiler.helper.binop(BINOP::LargerOrEq),
            "<" => compiler.helper.binop(BINOP::LessThan),
            "<=" => compiler.helper.binop(BINOP::LessOrEq),
            "!=" => compiler.helper.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };
        if checked_type.is_none() {
            errors::math_error(&mut compiler.helper, node);
            exit(0);
        }
        match sign {
            "=" => compiler.block.binop(BINOP::Eq),
            ">" => compiler.block.binop(BINOP::LargerThan),
            ">=" => compiler.block.binop(BINOP::LargerOrEq),
            "<" => compiler.block.binop(BINOP::LessThan),
            "<=" => compiler.block.binop(BINOP::LessOrEq),
            "!=" => compiler.block.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        }
    } else if title == "un" {
        parse_ast(node.child(0), compiler);
        parse_ast(node.child(1), compiler);
        compiler.block.binop(BINOP::And);
        compiler.helper.binop(BINOP::And);
    } else if title == "vai" {
        parse_ast(node.child(0), compiler);
        parse_ast(node.child(1), compiler);
        compiler.block.binop(BINOP::Or);
        compiler.helper.binop(BINOP::Or);
    } else if title == "xvai" {
        parse_ast(node.child(0), compiler);
        parse_ast(node.child(1), compiler);
        compiler.block.binop(BINOP::Xor);
        compiler.helper.binop(BINOP::Xor);
    }
}
