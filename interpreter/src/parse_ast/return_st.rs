use celsium::{ block::Block, compile_time_checker::CompileTimeChecker };
use hime_redist::ast::AstNode;

use super::parse_ast;

pub fn return_st(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool
) {
    if title == "return_st" {
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.return_from_function();
    }
}
