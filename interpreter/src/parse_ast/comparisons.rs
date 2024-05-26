use std::process::exit;

use celsium::{ block::Block, bytecode::BINOP, compile_time_checker::CompileTimeChecker };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use super::parse_ast;

pub fn comparisons(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool
) {
    if title == "comp_s" {
        let sign = node.child(1).get_value().unwrap();
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(2), block, is_wasm, typestack);
        let checked_type = match sign {
            "=" => typestack.binop(BINOP::EQ),
            ">" => typestack.binop(BINOP::LargerThan),
            ">=" => typestack.binop(BINOP::LargerOrEq),
            "<" => typestack.binop(BINOP::LessThan),
            "<=" => typestack.binop(BINOP::LessOrEq),
            "!=" => typestack.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };
        if checked_type.is_none() {
            crate::errors::math_error(
                "Ar šiem datu tipiem nevar veikt šo matemātisko darbību",
                &typestack.source_files[typestack.current_file],
                &typestack.source_file_paths[typestack.current_file],
                node.child(0).get_position().unwrap().line,
                node.child(0).get_position().unwrap().column,
            );
            exit(0);
        }
        match sign {
            "=" => block.binop(BINOP::EQ),
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
        block.binop(BINOP::AND);
        typestack.binop(BINOP::AND);
    } else if title == "vai" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.binop(BINOP::OR);
        typestack.binop(BINOP::OR);

    } else if title == "xvai" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.binop(BINOP::XOR);
        typestack.binop(BINOP::XOR);

    }
}
