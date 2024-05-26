use std::process::exit;

use celsium::{ block::Block, compile_time_checker::CompileTimeChecker, BUILTIN_TYPES };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::errors;

use super::parse_ast;

pub fn var_def(
    node: AstNode,
    title: String,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool,
    block: &mut Block
) {
    let current_file = typestack.source_files[typestack.current_file].clone();
    let current_file_path = typestack.source_file_paths[typestack.current_file].clone();
    if title == "var_def" {
        if node.child(1).get_symbol().to_string() == "ARRAY" {
            let array_name = node.child(2).get_value().unwrap();
            //user marked data type
            let data_type_marked = match node.child(0).to_string().as_str() {
                "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                "TEXT" => celsium::BUILTIN_TYPES::STRING,
                "FLOAT" => celsium::BUILTIN_TYPES::FLOAT,
                _ => panic!(),
            };

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
                        &current_file,
                        &current_file_path,
                        node.child(1).get_position().unwrap().line,
                        node.child(1).get_position().unwrap().column
                    );
                }
                init_value_counter += 1;
            }
            typestack.def_array(array_name, data_type_marked, node.child(3).children().len());
            block.define_array(
                celsium::module::VISIBILITY::PRIVATE,
                array_name.to_string(),
                node.child(3).children().len()
            )
        } else {
            //user marked data type
            let data_type_marked = match node.child(0).to_string().as_str() {
                "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                "TEXT" => celsium::BUILTIN_TYPES::STRING,
                "FLOAT" => celsium::BUILTIN_TYPES::FLOAT,
                _ => panic!(),
            };
            //parse the init value
            parse_ast(node.child(2), block, is_wasm, typestack);
            //get they type of the init value
            let typ_of_init_value = typestack.pop();
            if typ_of_init_value.clone().unwrap() != data_type_marked {
                errors::incorect_init_value(
                    format!(
                        "Mainīgā datu tips ir norādīts kā `{}`, bet piešķirtā sākotnējā vērtība ir `{}`.",
                        match node.child(0).to_string().as_str() {
                            "NUM" => "skaitlis",
                            "BOOL_DEF" => "būls",
                            "TEXT" => "tekts",
                            "FLOAT" => "decimālskaitlis",
                            _ => panic!(),
                        },
                        match typ_of_init_value.unwrap() {
                            BUILTIN_TYPES::MAGIC_INT => "skaitlis",
                            BUILTIN_TYPES::BOOL => "būls",
                            BUILTIN_TYPES::STRING => "teksts",
                            BUILTIN_TYPES::OBJECT => "objekts",
                            BUILTIN_TYPES::FLOAT => "decimālskaitlis",
                        }
                    ),
                    &typestack.source_files[typestack.current_file],
                    &typestack.source_file_paths[typestack.current_file],
                    node.child(1).get_position().unwrap().line,
                    node.child(1).get_position().unwrap().column
                );
                exit(0);
            }
            typestack.def_var(
                node.child(1).get_value().unwrap().to_string(),
                data_type_marked.clone()
            );
            block.define_variable(
                data_type_marked,
                celsium::module::VISIBILITY::PRIVATE,
                node.child(1).get_value().unwrap()
            )
        }
    }
}
