use celsium::{ block::Block, compiletime_helper::CompileTimeHelper };
use hime_redist::ast::AstNode;

use super::parse_ast;

pub fn return_st(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "return_st" {
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.return_from_function();
    }
}
