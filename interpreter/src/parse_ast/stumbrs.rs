use celsium::{block::Block, compile_time_checker::CompileTimeChecker, module::VISIBILITY, BUILTIN_TYPES};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};
use stumbrs::{StumbrsArrayDataTypes, StumbrsArrayValue, StumbrsValue};

use crate::get_stumbrs_data;

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn stumbrs_define(node: AstNode, block: &mut Block, typestack: &mut CompileTimeChecker, is_wasm: bool){
    if node.child(1).child(0).get_value().unwrap() != "STUMBRS" {
            panic!("Šeit var izmantot tikai 'STUMBRS' funkciju.");
        }
        let path = node
            .child(1)
            .child(1)
            .child(0)
            .get_value()
            .unwrap()
            .to_owned();
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
                    StumbrsValue::Array { value } => values =  value,
                    StumbrsValue::SimpleValue { value:_ } => panic!(),
                }
                for val in &values{
                    
                    block.load_const(match val.data_type {
                        StumbrsArrayDataTypes::MAGIC_INT => BUILTIN_TYPES::MAGIC_INT,
                        StumbrsArrayDataTypes::BOOL => BUILTIN_TYPES::BOOL,
                        StumbrsArrayDataTypes::STRING => BUILTIN_TYPES::STRING,
                        StumbrsArrayDataTypes::OBJECT => BUILTIN_TYPES::OBJECT,
                        StumbrsArrayDataTypes::FLOAT => BUILTIN_TYPES::FLOAT,
                    }, &val.value);
                }
                block.define_array(
                    VISIBILITY::PRIVATE,
                    node.child(0)
                        .child(counter)
                        .get_value()
                        .unwrap()
                        .to_string(),
                    values.len(),
                )
            } else {
                let data_type = match unit.data_type.as_str() {
                    "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                    "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                    "TEXT" => celsium::BUILTIN_TYPES::STRING,
                    _ => panic!(),
                };
                if unit.data_type.as_str() == "TEXT" {
                    block.load_const(data_type.clone(), rem_first_and_last(match &unit.value {
                        StumbrsValue::SimpleValue { value } => &value,
                        StumbrsValue::Array { value } => panic!(),
                    } ));
                } else {
                    block.load_const(data_type.clone(), match &unit.value {
                        StumbrsValue::SimpleValue { value } => &value,
                        StumbrsValue::Array { value } => panic!(),
                    } );
                }
                block.define_variable(
                    data_type,
                    VISIBILITY::PRIVATE,
                    node.child(0).child(counter).get_value().unwrap(),
                );
            }

            counter += 1;
        }
}