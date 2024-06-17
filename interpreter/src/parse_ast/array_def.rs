use std::process::exit;

use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope, BUILTIN_TYPES };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ errors, util::{ self, get_closest_block } };

use super::{ array_def, parse_ast };

pub fn array_def(
    node: AstNode,
    title: &str,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool,
    block: &mut Block,
    is_exported: bool
) {
    let array_name = node
        .child(2 + (is_exported as usize))
        .get_value()
        .unwrap();
    //user marked data type
    let data_type_str = node
        .child(0 + (is_exported as usize))
        .get_value().unwrap();
    let data_type_marked_option = util::data_type_from_str(data_type_str);

    let mut data_type_marked: BUILTIN_TYPES;

    if data_type_marked_option.is_none() {
        let struct_exists = typestack.struct_exists(data_type_str);
        if struct_exists.is_some() {
            data_type_marked = BUILTIN_TYPES::OBJECT {
                name: struct_exists.clone().unwrap().name,
                fields: struct_exists.unwrap().fields,
            };
        }
        else{
            errors::notexistant_type(
                data_type_str.to_owned(),
                node,
                typestack
            );
            panic!();
        }
    }
    else{
        data_type_marked = data_type_marked_option.unwrap();
    }

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
