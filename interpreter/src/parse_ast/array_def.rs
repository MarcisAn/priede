use celsium::{ block::Block, compiletime_helper::CompileTimeHelper };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ errors, util::get_data_type_from_id };

use super::parse_ast;

pub fn array_def(
    node: AstNode,
    title: &str,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool,
    block: &mut Block,
    is_exported: bool
) {
    if title == "array_def" {
        let array_name = node
            .child(2 + (is_exported as usize))
            .get_value()
            .unwrap();
        //user marked data type
        let data_type_str = node
            .child(0 + (is_exported as usize))
            .get_value()
            .unwrap();

        let data_type_marked = get_data_type_from_id(typestack, data_type_str, node);

        let mut init_value_counter = 0;
        for i in node.child(3 + (is_exported as usize)).children() {
            parse_ast(i, block, is_wasm, typestack);
            let type_of_init_val = typestack.pop();
            if type_of_init_val.clone().unwrap() != data_type_marked.clone() {
                errors::array_element_wrong_type(
                    array_name.to_owned(),
                    init_value_counter,
                    data_type_marked.clone(),
                    type_of_init_val.unwrap().clone(),
                    typestack,
                    node
                );
            }
            init_value_counter += 1;
        }
        let var_id = typestack.def_array(
            array_name,
            data_type_marked,
            node
                .child(3 + (is_exported as usize))
                .children()
                .len(),
            block.scope.clone(),
            is_exported
        );
        block.define_array(
            node
                .child(3 + (is_exported as usize))
                .children()
                .len(),
            var_id
        )
    }
}
