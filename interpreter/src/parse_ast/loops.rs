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
        let mut loop_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});
        let mut main_block = compiler.block.clone();
        compiler.block = loop_block.clone();
        parse_ast(node.child(1), compiler);
        compiler.block = main_block;
        compiler.block.define_simple_loop(loop_block);
    } else if title == "w_loop" {
        let current_module_path = compiler.helper.source_file_paths[compiler.helper.current_file].clone();
        let mut loop_block = Block::new(Scope{ast_id: node.child(1).id(), module_path: current_module_path.clone()});
        let mut conditional_block = Block::new(Scope{ast_id: node.child(0).id(), module_path: current_module_path});
        let mut main_block = compiler.block.clone();
        compiler.block = conditional_block.clone();
        parse_ast(node.child(0), compiler);
        compiler.block = loop_block.clone();
        parse_ast(node.child(1), compiler);
        compiler.block = main_block;
        compiler.block.define_while_loop(loop_block, conditional_block);
    }
}
