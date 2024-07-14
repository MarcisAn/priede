use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ hime, util, Compiler };

use super::parse_ast;

pub fn include(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler
) {
    if title == "include" {
        let include_source = node.child(2).get_value().unwrap();
        if include_source.starts_with("\"") {
            //is path
            let path = util::rem_first_and_last(include_source).to_string();
            let file_content = crate::read_file(path.clone());
            let parse_res = hime::priede::parse_string(file_content.clone());
            crate::parser::parser_errors(parse_res.errors.clone().errors);

            let ast = parse_res.get_ast();
            let root = ast.get_root();

            compiler.helper.change_module(file_content, path.clone());
            let mut module_main_block = Block::new(Scope {
                ast_id: root.id(),
                module_path: path.clone(),
            });
            let mut main_block = compiler.block.clone();
            compiler.block = module_main_block.clone();
            parse_ast(root, compiler);
            compiler.block = main_block.clone();
            compiler.helper.switch_to_prev_module();

            for import in node.child(0).children() {
                compiler.helper.import(
                    import.get_value().unwrap().to_string(),
                    path.clone(),
                    compiler.block.scope.clone().module_path
                );
            }

            compiler.block.add_blocks_bytecode(module_main_block);
        } else {
            //is library
        }
    }
}
