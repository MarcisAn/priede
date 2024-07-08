use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope };
use hime_redist::ast::AstNode;

use crate::Compiler;

use super::parse_ast;

pub fn if_stat(node: AstNode, title: &str, compiler: &mut Compiler) {
    if title == "if" {
        let current_module_path = compiler.helper.source_file_paths[compiler.helper.current_file].clone();

        parse_ast(node.child(0), compiler);
        let mut main_block = compiler.block.clone();
        let mut if_block = Block::new(Scope {
            ast_id: node.child(1).id(),
            module_path: current_module_path.clone(),
        });
        compiler.block = if_block.clone();
        parse_ast(node.child(1), compiler);
        if node.children_count() > 2 {
            let mut else_block = Block::new(Scope {
                ast_id: node.child(3).id(),
                module_path: current_module_path,
            });
            compiler.block = else_block.clone();
            parse_ast(node.child(3), compiler);
            compiler.block = main_block.clone();
            main_block.define_if_else_block(if_block, else_block)
        } else {
            compiler.block = main_block.clone();
            main_block.define_if_block(if_block);
        }
    }
}
