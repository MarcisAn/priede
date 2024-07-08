use celsium::{ block::Block, compiletime_helper::CompileTimeHelper };
use hime_redist::ast::AstNode;

use crate::Compiler;

use super::parse_ast;

pub fn return_st(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler
) {
    if title == "return_st" {
        parse_ast(node.child(1), compiler);
        compiler.block.return_from_function();
    }
}
