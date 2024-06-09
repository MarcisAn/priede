use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, BUILTIN_TYPES };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{errors, util::{self, get_closest_scope}};

use super::parse_ast;

pub fn array(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "array" {
        let array_name = node.child(0).get_value().unwrap();
        let array = util::get_closest_scope(array_name.to_string(), block.ast_id, typestack, node);
        if array.is_none() {
            errors::undefined_var(
                format!("Saraksts `{}` nav definēts", array_name),
                typestack,
                node
            );
        }
        //parse the index
        parse_ast(node.child(1), block, is_wasm, typestack);
        let index_type = typestack.pop().unwrap();
        if index_type != BUILTIN_TYPES::MAGIC_INT {
            errors::array_element_wrong_type_index(
                array_name.to_string(),
                BUILTIN_TYPES::MAGIC_INT,
                index_type,
                typestack,
                node
            );
        }
        
        let array_type_anf_len = typestack.get_array_type_and_length(array.unwrap()).unwrap();
        let array_length = array_type_anf_len.1;
        let array_type = array_type_anf_len.0;

        //extra check if the index is number
        if node.child(1).get_symbol().to_string() == "NUMBER" {
            let index_number: usize = node.child(1).get_value().unwrap().parse().unwrap();
            if array_length - 1 < index_number {
                errors::array_element_index_too_high(
                    array_name.to_string(),
                    array_length,
                    index_number,
                    typestack,
                    node
                );
            }
        }

        typestack.push(array_type);
        block.load_from_array(array.unwrap());
    }
}
