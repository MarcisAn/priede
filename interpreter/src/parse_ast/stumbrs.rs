use celsium::{
    block::Block,
    compiletime_helper::CompileTimeHelper,
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use stumbrs::{ StumbrsArrayDataTypes, StumbrsArrayValue, StumbrsValue };

use crate::get_stumbrs_data;

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn stumbrs_define(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    /*
    if title == "multiple_id_define" {
        if node.child(1).child(0).get_value().unwrap() != "STUMBRS" {
            panic!("Šeit var izmantot tikai 'STUMBRS' funkciju.");
        }
        let path = node.child(1).child(1).child(0).get_value().unwrap().to_owned();
        let mut chars = path.chars();
        chars.next();
        chars.next_back();
        let data = match is_wasm {
            false => stumbrs::load_stumbrs_data_file(chars.as_str().to_string()),
            true => stumbrs::load_stumbrs_data(get_stumbrs_data()),
        };
        let mut counter = 0;
        for unit in data.units {
            if unit.data_type.as_str() == "[]" {
                let values: Vec<StumbrsArrayValue>;
                match unit.value {
                    StumbrsValue::Array { value } => {
                        values = value;
                    }
                    StumbrsValue::SimpleValue { value: _ } => panic!(),
                }
                for val in &values {
                    block.load_const(
                        match val.data_type {
                            StumbrsArrayDataTypes::MAGIC_INT => BUILTIN_TYPES::MAGIC_INT,
                            StumbrsArrayDataTypes::BOOL => BUILTIN_TYPES::BOOL,
                            StumbrsArrayDataTypes::STRING => BUILTIN_TYPES::STRING,
                            StumbrsArrayDataTypes::OBJECT => BUILTIN_TYPES::OBJECT,
                            StumbrsArrayDataTypes::FLOAT => BUILTIN_TYPES::FLOAT,
                        },
                        &val.value
                    );
                }
                //TODO: typed arrays in stumbrs
                /*typestack.def_array(node.child(0).child(counter).get_value().unwrap(), data_type, values.len());
                block.define_array(
                    VISIBILITY::PRIVATE,
                    ,
                    values.len(),
                    block.ast_id
                );*/
            } else {
                let data_type = match unit.data_type.as_str() {
                    "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                    "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                    "TEXT" => celsium::BUILTIN_TYPES::STRING,
                    "FLOAT" => celsium::BUILTIN_TYPES::FLOAT,
                    _ => panic!(),
                };
                if unit.data_type.as_str() == "TEXT" {
                    block.load_const(
                        data_type.clone(),
                        rem_first_and_last(match &unit.value {
                            StumbrsValue::SimpleValue { value } => &value,
                            StumbrsValue::Array { value: _ } => panic!(),
                        })
                    );
                } else {
                    block.load_const(data_type.clone(), match &unit.value {
                        StumbrsValue::SimpleValue { value } => &value,
                        StumbrsValue::Array { value: _ } => panic!(),
                    });
                }
                let var_id = typestack.def_var(
                    node.child(0).child(counter).get_value().unwrap().to_string(),
                    data_type.clone(),
                    block.scope.clone(),
                    false
                );
                block.define_variable(var_id.unwrap());
            }

            counter += 1;
        }
        }*/


}