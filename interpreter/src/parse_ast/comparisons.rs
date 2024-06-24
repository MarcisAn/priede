use std::process::exit;

use celsium::{ block::Block, bytecode::BINOP, compiletime_helper::CompileTimeHelper };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::errors;

use super::parse_ast;

pub fn comparisons(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "comp_s" {
        let sign = node.child(1).get_value().unwrap();
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(2), block, is_wasm, typestack);
        let checked_type = match sign {
            "=" => typestack.binop(BINOP::Eq),
            ">" => typestack.binop(BINOP::LargerThan),
            ">=" => typestack.binop(BINOP::LargerOrEq),
            "<" => typestack.binop(BINOP::LessThan),
            "<=" => typestack.binop(BINOP::LessOrEq),
            "!=" => typestack.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };
        if checked_type.is_none() {
            errors::math_error(
                typestack,
                node
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
    }
    else if title == "un" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.binop(BINOP::And);
        typestack.binop(BINOP::And);
    } else if title == "vai" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.binop(BINOP::Or);
        typestack.binop(BINOP::Or);

    } else if title == "xvai" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.binop(BINOP::Xor);
        typestack.binop(BINOP::Xor);

    }
}
