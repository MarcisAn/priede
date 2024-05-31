use celsium::{ block::Block, compile_time_checker::CompileTimeChecker };
use hime_redist::ast::AstNode;


use super::parse_ast;

pub fn if_stat(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool
) {
    if title == "if" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        let mut if_block = Block::new(node.child(1).id());
        parse_ast(node.child(1), &mut if_block, is_wasm, typestack);
        if node.children_count() > 2 {
            let mut else_block = Block::new(node.child(3).id());
            parse_ast(node.child(3), &mut else_block, is_wasm, typestack);
            block.define_if_else_block(if_block, else_block)
        } else {
            block.define_if_block(if_block);
        }
    }
}
