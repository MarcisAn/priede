use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope };
use hime_redist::ast::AstNode;


use super::parse_ast;

pub fn if_stat(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "if" {
        let current_module_path = typestack.source_file_paths[typestack.current_file].clone();

        parse_ast(node.child(0), block, is_wasm, typestack);
        let mut if_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});
        parse_ast(node.child(1), &mut if_block, is_wasm, typestack);
        if node.children_count() > 2 {
            let mut else_block = Block::new(Scope{ast_id: node.child(3).id(), module_path: current_module_path});
            parse_ast(node.child(3), &mut else_block, is_wasm, typestack);
            block.define_if_else_block(if_block, else_block)
        } else {
            block.define_if_block(if_block);
        }
    }
}
