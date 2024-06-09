use celsium::{ block::Block, compiletime_helper::CompileTimeHelper };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, util };

use super::parse_ast;

pub fn func_call(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "func_call" {
        if node.children_count() > 1 {
            for arg in node.child(1) {
                parse_ast(arg, block, is_wasm, typestack);
            }
        }
        let func_name = node.child(0).get_value().unwrap();
        if func_name == "izvade" {
            block.call_special_function(celsium::SpecialFunctions::PRINT { newline: true });
        } else if func_name == "izvadetp" {
            block.call_special_function(celsium::SpecialFunctions::PRINT { newline: false });
        } else if func_name == "ievade" {
            block.call_special_function(celsium::SpecialFunctions::INPUT);
        } else if func_name == "jukums" {
            parse_ast(node.child(1).child(0), block, is_wasm, typestack);
            parse_ast(node.child(1).child(1), block, is_wasm, typestack);
            block.call_special_function(celsium::SpecialFunctions::RANDOM);
        } else {
            let func_id = util::get_closest_scope(
                func_name.to_string(),
                block.ast_id,
                typestack,
                node
            );
            if func_id.is_none() {
                errors::undefined_func(
                    format!("Funkcija `{}` nav definēta", func_name),
                    &typestack.source_files[typestack.current_file],
                    &typestack.source_file_paths[typestack.current_file],
                    util::get_closest_node_location(node)
                );
            }
            let func_return_type = typestack.get_func_return_type(func_id.unwrap()).unwrap();
            typestack.push(func_return_type);
            block.call_function(func_name);
        }
    }
}
