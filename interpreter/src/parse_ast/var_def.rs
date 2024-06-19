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
            let data_type_marked = util::get_data_type_from_id(typestack, data_type_str, node);
            
            //parse the init value
            parse_ast(node.child(2 + (is_exported as usize)), block, is_wasm, typestack);
            
            //get they type of the init value
            let typ_of_init_value = typestack.pop().unwrap();
            //println!("type comparison real {:?} marked {:?}", typ_of_init_value, data_type_marked);
            
            
            if typ_of_init_value.clone() != data_type_marked {
                errors::incorect_init_value(
                    format!(
                        "Mainīgā datu tips ir norādīts kā `{}`, bet piešķirtā sākotnējā vērtība ir `{}`.",
                        data_type_str,
                        util::str_from_data_type(typ_of_init_value)
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
