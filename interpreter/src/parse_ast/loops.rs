use celsium::{ block::{self, Block}, compiletime_helper::CompileTimeHelper, Scope };
use hime_redist::ast::AstNode;

use crate::Compiler;

use super::parse_ast;

pub fn loops(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler, block: &mut Block
) {
    if title == "s_loop" {
        let current_module_path = compiler.helper.source_file_paths[compiler.helper.current_file].clone();
        parse_ast(node.child(0), compiler, block);
        let mut loop_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});
        parse_ast(node.child(1), compiler, &mut loop_block);
        block.define_simple_loop(loop_block);
    } else if title == "w_loop" {
        let current_module_path = compiler.helper.source_file_paths[compiler.helper.current_file].clone();
        let mut loop_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});
        let mut conditional_block = Block::new(Scope{ast_id: node.child(0).id(), module_path: current_module_path});
        parse_ast(node.child(0), compiler, &mut conditional_block);
        parse_ast(node.child(1), compiler, &mut loop_block);
        block.define_while_loop(loop_block, conditional_block);
    }
}
