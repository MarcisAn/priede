use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope };
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};


use crate::{hime, util};

use super::parse_ast;

pub fn include(
    node: AstNode,
    title: &str,
    is_wasm: bool,
    typestack: &mut CompileTimeHelper,
    block: &mut Block
) {
    if title == "include" {
        let path = util::rem_first_and_last(node.child(2).get_value().unwrap()).to_string();
        let file_content = crate::read_file(path.clone());
        let parse_res = hime::priede::parse_string(file_content.clone());
        crate::parser_errors(parse_res.errors.clone().errors, typestack);

        let ast = parse_res.get_ast();
        let root = ast.get_root();

        typestack.change_module(file_content, path.clone());
        let mut module_main_block = Block::new(Scope { ast_id: root.id(), module_path: path.clone() });
        parse_ast(root, &mut module_main_block, is_wasm, typestack);
        typestack.switch_to_prev_module();

        for import in node.child(0).children() {
            typestack.import(
                import.get_value().unwrap().to_string(),
                path.clone(),
                block.scope.clone().module_path
            );
        }

        block.add_blocks_bytecode(module_main_block);
    }
}
