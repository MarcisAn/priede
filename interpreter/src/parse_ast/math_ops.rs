use std::process::exit;

use crate::{errors::math_error, util};
use celsium::{block::Block, bytecode::BINOP, compiletime_helper::CompileTimeHelper};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use super::parse_ast;

pub fn math_ops(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
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
    typestack: &mut CompileTimeHelper,
    is_wasm: bool,
) {
    parse_ast(node.child(0), block, is_wasm, typestack);
    parse_ast(node.child(1), block, is_wasm, typestack);
    let res = typestack.binop(binop.clone());
    if res.is_none() {
        math_error(typestack, node);
        exit(0);
    }
    block.binop(binop);

    
}
