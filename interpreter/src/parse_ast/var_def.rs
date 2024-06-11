use std::process::exit;

use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, Scope, BUILTIN_TYPES };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{errors, util::{self, get_closest_block}};

use super::parse_ast;

pub fn var_def(
    node: AstNode,
    title: &str,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool,
    block: &mut Block
) {
    if title == "var_def" {
        if node.child(1).get_symbol().to_string() == "ARRAY" {
            let array_name = node.child(2).get_value().unwrap();
            //user marked data type
            let data_type_marked = util::data_type_from_str(node.child(0).to_string().as_str());

            let mut init_value_counter = 0;
            for i in node.child(3).children() {
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
            let var_id = typestack.def_array(array_name, data_type_marked, node.child(3).children().len(), block.scope.clone());
            block.define_array(
                node.child(3).children().len(),
                var_id
            )
        } else {
            //user marked data type
            let data_type_marked = util::data_type_from_str(node.child(0).to_string().as_str());
            //parse the init value
            parse_ast(node.child(2), block, is_wasm, typestack);
            //get they type of the init value
            let typ_of_init_value = typestack.pop();
            if typ_of_init_value.clone().unwrap() != data_type_marked {
                errors::incorect_init_value(
                    format!(
                        "Mainīgā datu tips ir norādīts kā `{}`, bet piešķirtā sākotnējā vērtība ir `{}`.",
                        util::str_from_data_type(util::data_type_from_str(node.child(0).to_string().as_str())),
                        util::str_from_data_type(typ_of_init_value.unwrap())
                    ),
                    typestack,
                    node
                );
                exit(0);
            }
            let varname = node.child(1).get_value().unwrap().to_string();
            let var_id = typestack.def_var(
                varname,
                data_type_marked.clone(),
                block.scope.clone()
            );
            block.define_variable(
                var_id,
            );
        }
    }
}
