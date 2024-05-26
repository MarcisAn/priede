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
        let mut loop_block = Block::new();
        let mut loop_count_block = Block::new();

        parse_ast(node.child(1), &mut loop_block, is_wasm, typestack);
        parse_ast(node.child(0), &mut loop_count_block, is_wasm, typestack);
        block.define_simple_loop(loop_block, loop_count_block);
    } else if title == "w_loop" {
        let mut loop_block = Block::new();
        let mut conditional_block = Block::new();
        parse_ast(node.child(0), &mut conditional_block, is_wasm, typestack);
        parse_ast(node.child(1), &mut loop_block, is_wasm, typestack);
        block.define_while_loop(loop_block, conditional_block);
    }
}
