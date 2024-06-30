use std::process::exit;

use celsium::{ block::Block, compiletime_helper::CompileTimeHelper };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ errors, util::{ self, get_object_fields } };

use super::{ array_def, parse_ast };

pub fn var_def(
    node: AstNode,
    title: &str,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool,
    block: &mut Block
) {
    if title == "var_def" || title == "array_def"{
        let is_exported = node.child(0).get_symbol().to_string() == "EXPORT";
        if node.child(1 + is_exported as usize).get_symbol().to_string() == "ARRAY" {
            array_def::array_def(node, title, typestack, is_wasm, block, is_exported);
        } else {
            //user marked data type
            let data_type_str = node
                .child(0 + (is_exported as usize))
                .get_value()
                .unwrap();
            let data_type_marked = util::get_data_type_from_id(typestack, data_type_str, node);

            //parse the init value
            parse_ast(node.child(2 + (is_exported as usize)), block, is_wasm, typestack);

            //get they type of the init value
            let typ_of_init_value = typestack.pop().unwrap();
            //println!("type comparison real {:?} marked {:?}", typ_of_init_value, data_type_marked);

            let mut should_objects_error = false;

            let are_object_types_eq = util::compare_object_types(
                &typ_of_init_value,
                &data_type_marked
            );


            if are_object_types_eq.is_ok() {
                should_objects_error = !are_object_types_eq.unwrap();
            }

            if typ_of_init_value.clone() != data_type_marked && should_objects_error {
                errors::incorect_init_value(
                    format!(
                        "Mainīgā datu tips ir norādīts kā `{}`, bet piešķirtā sākotnējā vērtība ir `{}`.",
                        data_type_str,
                        util::str_from_data_type(typ_of_init_value)
                    ),
                    typestack,
                    node.child(0 + (is_exported as usize))
                );
                exit(0);
            }
            let varname = node
                .child(1 + (is_exported as usize))
                .get_value()
                .unwrap()
                .to_string();

            let mut typestact_copy = typestack.clone();

            let is_object = util::is_type_object(&typ_of_init_value);

            
            if is_object {
                let fields = get_object_fields(&typ_of_init_value).unwrap();
                
                let object_id = typestack.def_object(
                    varname.clone(),
                    block.scope.clone(),
                    is_exported,
                    fields.clone(),
                );
                if object_id.is_err() {
                    if object_id.err().unwrap() == "already_defined" {
                        errors::incorect_init_value(
                            format!("Mainīgais `{}` jau ir definēts.", varname),
                            &mut typestact_copy,
                            node.child(2 + (is_exported as usize))
                        );
                    }
                    if object_id.err().unwrap() == "already_imported" {
                        errors::incorect_init_value(
                            format!("Mainīgais `{}` jau ir iekļauts.", varname),
                            &mut typestact_copy,
                            node.child(2 + (is_exported as usize))
                        );
                    }
                }
                let mut field_names: Vec<String> = vec![];
                for field in fields.clone(){
                    field_names.push(field.name);
                }
                block.define_object(object_id.unwrap());
            } else {
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
}
