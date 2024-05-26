use std::process::exit;

use crate::errors::math_error;
use celsium::{block::Block, bytecode::BINOP, compile_time_checker::CompileTimeChecker};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use super::parse_ast;

pub fn math_ops(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool
){
    if title == "plus"
        || title == "minus"
        || title == "reiz"
        || title == "dal"
        || title == "atlik"
    {
        calculate(
            match title {
                "plus" => BINOP::ADD,
                "minus" => BINOP::SUBTRACT,
                "reiz" => BINOP::MULTIPLY,
                "dal" => BINOP::DIVIDE,
                "atlik" => BINOP::REMAINDER,
                _ => panic!(),
            },
            node,
            block,
            typestack,
            is_wasm,
        );
    }
}
fn calculate(
    binop: BINOP,
    node: AstNode,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool,
) {
    parse_ast(node.child(0), block, is_wasm, typestack);
    parse_ast(node.child(1), block, is_wasm, typestack);
    let res = typestack.binop(BINOP::ADD);
    if res.is_none() {
        math_error(
            "Ar šiem datu tipiem nevar veikt šo matemātisko darbību",
            &typestack.source_files[typestack.current_file],
            &typestack.source_file_paths[typestack.current_file],
            node.child(0).get_position().unwrap().line,
            node.child(0).get_position().unwrap().column,
        );
        exit(0);
    }
    block.binop(binop);

    
}
