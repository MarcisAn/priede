use celsium::{ block::Block, Scope };
use hime_redist::ast::AstNode;

use crate::{util::get_furthest_node_location, Compiler};

use super::parse_ast;

pub fn if_stat(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "if" {
        let current_module_path =
            compiler.helper.source_file_paths[compiler.helper.current_file].clone();

        parse_ast(node.child(0), compiler, block);
        let mut if_block = Block::new(Scope {
            ast_id: node.child(1).id(),
            module_path: current_module_path.clone(),
        });
        parse_ast(node.child(1), compiler, &mut if_block);
        let end_of_block = get_furthest_node_location(node).unwrap();
        if node.children_count() > 3 {
            let mut else_block = Block::new(Scope {
                ast_id: node.child(3).id(),
                module_path: current_module_path,
            });
            parse_ast(node.child(4), compiler, &mut else_block);

            block.define_if_else_block(if_block, else_block, end_of_block.line, end_of_block.column);
        } else {
            block.define_if_block(if_block, end_of_block.line, end_of_block.column);
        }
    }
}
