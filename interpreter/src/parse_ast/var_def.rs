use std::process::exit;

use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope, BUILTIN_TYPES };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ errors, util::{ self, get_closest_block } };

use super::{ array_def, parse_ast };

pub fn var_def(
    node: AstNode,
    title: &str,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool,
    block: &mut Block
) {
    if title == "var_def" {
        let is_exported = node.child(0).get_symbol().to_string() == "EXPORT";
        if node.child(1).get_symbol().to_string() == "ARRAY" {
            array_def::array_def(node, title, typestack, is_wasm, block, is_exported);
        } else {
            //user marked data type
            let data_type_str = node
                .child(0 + (is_exported as usize)).get_value().unwrap();
            let data_type_marked_option = util::data_type_from_str(data_type_str);

            let mut data_type_marked: BUILTIN_TYPES;
            if data_type_marked_option.is_none() {
                let struct_exists = typestack.struct_exists(data_type_str);
                if struct_exists.is_some() {
                    data_type_marked = BUILTIN_TYPES::OBJECT {
                        name: struct_exists.clone().unwrap().name,
                        fields: struct_exists.unwrap().fields,
                    };
                } else {
                    errors::notexistant_type(data_type_str.to_owned(), node, typestack);
                    panic!();//to get rid of undefined error. not needed, because error exits
                }
            }
            else{
                data_type_marked = data_type_marked_option.unwrap();
            }
            //parse the init value
            parse_ast(node.child(2 + (is_exported as usize)), block, is_wasm, typestack);
            //get they type of the init value
            let typ_of_init_value = typestack.pop();
            if typ_of_init_value.clone().unwrap() != data_type_marked {
                errors::incorect_init_value(
                    format!(
                        "Mainīgā datu tips ir norādīts kā `{}`, bet piešķirtā sākotnējā vērtība ir `{}`.",
                        data_type_str,
                        util::str_from_data_type(typ_of_init_value.unwrap())
                    ),
                    typestack,
                    node.child(2 + (is_exported as usize))
                );
                exit(0);
            }
            let varname = node
                .child(1 + (is_exported as usize))
                .get_value()
                .unwrap()
                .to_string();

            let mut typestact_copy = typestack.clone();

            let var_id = typestack.def_var(
                varname.clone(),
                data_type_marked.clone(),
                block.scope.clone(),
                is_exported
            );
            if var_id.is_err() {
                if var_id.err().unwrap() == "already_defined" {
                    errors::incorect_init_value(
                        format!("Mainīgais `{}` jau ir definēts.", varname),
                        &mut typestact_copy,
                        node.child(2 + (is_exported as usize))
                    );
                }
                if var_id.err().unwrap() == "already_imported" {
                    errors::incorect_init_value(
                        format!("Mainīgais `{}` jau ir iekļauts.", varname),
                        &mut typestact_copy,
                        node.child(2 + (is_exported as usize))
                    );
                }
            }
            block.define_variable(var_id.unwrap());
        }
    }
}
