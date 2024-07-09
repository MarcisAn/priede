use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope };
use hime_redist::ast::AstNode;

use crate::Compiler;

use super::parse_ast;

pub fn loops(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler
) {
    if title == "s_loop" {
        let current_module_path = compiler.helper.source_file_paths[compiler.helper.current_file].clone();
        parse_ast(node.child(0), compiler);
        let loop_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});
        let main_block = compiler.block.clone();
        compiler.block = loop_block.clone();
        parse_ast(node.child(1), compiler);
        let loop_block_populated = compiler.block.clone();
        compiler.block = main_block;
        compiler.block.define_simple_loop(loop_block_populated);
    } else if title == "w_loop" {
        let current_module_path = compiler.helper.source_file_paths[compiler.helper.current_file].clone();
        let loop_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});
        let conditional_block = Block::new(Scope{ast_id: node.child(0).id(), module_path: current_module_path});
        let main_block = compiler.block.clone();
        compiler.block = conditional_block;
        parse_ast(node.child(0), compiler);
        let conditional_block_populated = compiler.block.clone();
        compiler.block = loop_block.clone();
        parse_ast(node.child(1), compiler);
        let loop_block_populated = compiler.block.clone();
        compiler.block = main_block;
        compiler.block.define_while_loop(loop_block_populated, conditional_block_populated);
    }
}
