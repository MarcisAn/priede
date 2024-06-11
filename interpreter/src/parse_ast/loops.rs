use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope };
use hime_redist::ast::AstNode;

use super::parse_ast;

pub fn loops(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "s_loop" {
        let current_module_path = typestack.source_file_paths[typestack.current_file].clone();
        parse_ast(node.child(0), block, is_wasm, typestack);
        let mut loop_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});

        parse_ast(node.child(1), &mut loop_block, is_wasm, typestack);
        block.define_simple_loop(loop_block);
    } else if title == "w_loop" {
        let current_module_path = typestack.source_file_paths[typestack.current_file].clone();
        let mut loop_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});
        let mut conditional_block = Block::new(Scope{ast_id: node.child(0).id(), module_path: current_module_path});
        parse_ast(node.child(0), &mut conditional_block, is_wasm, typestack);
        parse_ast(node.child(1), &mut loop_block, is_wasm, typestack);
        block.define_while_loop(loop_block, conditional_block);
    }
}
