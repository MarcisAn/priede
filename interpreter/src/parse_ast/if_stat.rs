use celsium::{ block::Block, Scope };
use hime_redist::ast::AstNode;

use crate::Compiler;

use super::parse_ast;

pub fn if_stat(node: AstNode, title: &str, compiler: &mut Compiler) {
    if title == "if" {
        let current_module_path = compiler.helper.source_file_paths[compiler.helper.current_file].clone();

        parse_ast(node.child(0), compiler);
        let exp_reg = compiler.helper.get_top().unwrap().register_id;
        let main_block = compiler.block.clone();
        let if_block = Block::new(Scope {
            ast_id: node.child(1).id(),
            module_path: current_module_path.clone(),
        });
        compiler.block = if_block.clone();
        parse_ast(node.child(1), compiler);
        let if_block_populated = compiler.block.clone();
        //if node.children_count() > 2 {
        //    let else_block = Block::new(Scope {
        //        ast_id: node.child(3).id(),
        //        module_path: current_module_path,
        //    });
        //    compiler.block = else_block.clone();
        //    parse_ast(node.child(3), compiler);
        //    let else_block_populated = compiler.block.clone();
        //    compiler.block = main_block.clone();
        //    compiler.block.define_if_else_block(if_block_populated, else_block_populated);
        //} else {
            compiler.block = main_block.clone();
            compiler.block.define_if_block(if_block_populated, exp_reg);
        //}
    }
}
