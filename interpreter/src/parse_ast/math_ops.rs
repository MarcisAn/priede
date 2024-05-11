use celsium::{
    block::Block, bytecode::BINOP, compile_time_checker::CompileTimeChecker
};
use hime_redist::ast::AstNode;

use super::parse_ast;

pub fn math_ops(
    binop: BINOP,
    node: AstNode,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool,
) {
    parse_ast(node.child(0), block, is_wasm, typestack);
    parse_ast(node.child(1), block, is_wasm, typestack);
    let res = typestack.binop(BINOP::ADD);
    println!("math out{:?}", res);
    block.binop(binop);
}
