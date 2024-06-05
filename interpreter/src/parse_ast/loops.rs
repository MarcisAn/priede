use celsium::{ block::Block, compile_time_checker::CompileTimeChecker };
use hime_redist::ast::AstNode;

use super::parse_ast;

pub fn loops(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool
) {
    if title == "s_loop" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        let mut loop_block = Block::new(node.child(1).id());

        parse_ast(node.child(1), &mut loop_block, is_wasm, typestack);
        block.define_simple_loop(loop_block);
    } else if title == "w_loop" {
        let mut loop_block = Block::new(node.child(1).id());
        let mut conditional_block = Block::new(node.child(0).id());
        parse_ast(node.child(0), &mut conditional_block, is_wasm, typestack);
        parse_ast(node.child(1), &mut loop_block, is_wasm, typestack);
        block.define_while_loop(loop_block, conditional_block);
    }
}
