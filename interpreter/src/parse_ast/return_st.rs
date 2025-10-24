use celsium::block::Block;
use hime_redist::ast::AstNode;

use crate::Compiler;

use super::parse_ast;

pub fn return_st(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler, block: &mut Block
) {
    if title == "return_st" {
        parse_ast(node.child(1), compiler, block);
        block.return_from_function();
    }
}
