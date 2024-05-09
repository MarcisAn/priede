use core::panic;

use super::get_stumbrs_data;
use crate::ast::*;
use celsium::{
    block::Block,
    bytecode::BINOP,
    module::{FuncArg, FunctionSignature, Module, VISIBILITY},
    BUILTIN_TYPES,
};
use hime_redist::{
    ast::{Ast, AstNode},
    symbols::SemanticElementTrait,
};
use stumbrs::*;

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn parse_ast(node: AstNode, block: &mut Block, is_wasm: bool) {
    let title = node.get_symbol().to_string();
    if title == "func_call" {
        if node.children_count() > 1 {
            for arg in node.child(1) {
                parse_ast(arg, block, is_wasm);
            }
        }
        let func_name = node.child(0).get_value().unwrap();
        if func_name == "izvade" {
            block.call_special_function(celsium::SpecialFunctions::PRINT { newline: true });
        } else if func_name == "izvadetp" {
            block.call_special_function(celsium::SpecialFunctions::PRINT { newline: false });
        } else if func_name == "ievade" {
            block.call_special_function(celsium::SpecialFunctions::INPUT);
        } else if func_name == "jukums" {
            parse_ast(node.child(1).child(0), block, is_wasm);
            parse_ast(node.child(1).child(1), block, is_wasm);
            block.call_special_function(celsium::SpecialFunctions::RANDOM);
        } else {
            block.call_function(func_name);
        }
    } else if title == "multiple_id_define" {
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
    } else if title == "return_st" {
        parse_ast(node.child(1), block, is_wasm);
        block.return_from_function();
    } else if title == "var_def" {
        println!("{}", node.child(0).get_symbol().to_string());
        if node.child(0).get_symbol().to_string() == "ARRAY" {
            for i in node.child(2).children() {
                parse_ast(i, block, is_wasm);
            }
            block.define_array(
                celsium::module::VISIBILITY::PRIVATE,
                node.child(1).get_value().unwrap().to_string(),
                node.child(2).children().len(),
            )
        } else {
            parse_ast(node.child(2), block, is_wasm);
            block.define_variable(
                match node.child(0).to_string().as_str() {
                    "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                    "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                    "TEXT" => celsium::BUILTIN_TYPES::STRING,

                    _ => panic!(),
                },
                celsium::module::VISIBILITY::PRIVATE,
                node.child(1).get_value().unwrap(),
            )
        }
    } else if title == "func_def" {
        let mut body = Block::new();
        let mut args: Vec<FuncArg> = vec![];

        if node.children_count() > 2 {
            //when the function takes arguments
            parse_ast(node.child(2), &mut body, is_wasm);
            for arg in node.child(1).children() {
                args.push(FuncArg {
                    name: arg.child(1).get_value().unwrap().to_string(),
                    arg_type: match arg.child(0).to_string().as_str() {
                        "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                        "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                        "TEXT" => celsium::BUILTIN_TYPES::MAGIC_INT,
                        _ => panic!(),
                    },
                })
            }
        } else {
            parse_ast(node.child(1), &mut body, is_wasm);
        }

        block.define_function(
            body,
            VISIBILITY::PUBLIC,
            FunctionSignature {
                name: node.child(0).get_value().unwrap().to_string(),
                return_type: celsium::module::FunctionReturnType::NONE,
                args: args,
            },
        )
    } else if title == "array" {
        parse_ast(node.child(1), block, is_wasm);
        block.load_from_array(node.child(0).get_value().unwrap());
    } else if title.starts_with("ID") {
        block.load_variable(node.get_value().unwrap());
    } else if title == "plus" || title == "string_plus" {
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(1), block, is_wasm);
        block.binop(BINOP::ADD);
    } else if title == "minus" {
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(1), block, is_wasm);
        block.binop(BINOP::SUBTRACT);
    } else if title == "reiz" {
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(1), block, is_wasm);
        block.binop(BINOP::MULTIPLY);
    } else if title == "dal" {
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(1), block, is_wasm);
        block.binop(BINOP::DIVIDE);
    } else if title == "atlik" {
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(1), block, is_wasm);
        block.binop(BINOP::REMAINDER);
    } else if title == "if" {
        parse_ast(node.child(0), block, is_wasm);
        let mut if_block = Block::new();
        parse_ast(node.child(1), &mut if_block, is_wasm);
        if node.children_count() > 2 {
            let mut else_block = Block::new();
            parse_ast(node.child(3), &mut else_block, is_wasm);
            block.define_if_else_block(if_block, else_block)
        } else {
            block.define_if_block(if_block);
        }
    } else if title == "comp_s" {
        let sign = node.child(1).get_value().unwrap();
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(2), block, is_wasm);
        match sign {
            "=" => block.binop(BINOP::EQ),
            ">" => block.binop(BINOP::LargerThan),
            ">=" => block.binop(BINOP::LargerOrEq),
            "<" => block.binop(BINOP::LessThan),
            "<=" => block.binop(BINOP::LessOrEq),
            "!=" => block.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        }
    } else if title == "un" {
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(1), block, is_wasm);
        block.binop(BINOP::AND);
    } else if title == "vai" {
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(1), block, is_wasm);
        block.binop(BINOP::OR);
    } else if title == "xvai" {
        parse_ast(node.child(0), block, is_wasm);
        parse_ast(node.child(1), block, is_wasm);
        block.binop(BINOP::XOR);
    } else if title == "block" {
        for i in node.children() {
            parse_ast(i, block, is_wasm);
        }
    } else if title == "s_loop" {
        let mut loop_block = Block::new();
        let mut loop_count_block = Block::new();

        parse_ast(node.child(1), &mut loop_block, is_wasm);
        parse_ast(node.child(0), &mut loop_count_block, is_wasm);
        block.define_simple_loop(loop_block, loop_count_block);
    } else if title == "w_loop" {
        let mut loop_block = Block::new();
        let mut conditional_block = Block::new();
        parse_ast(node.child(0), &mut conditional_block, is_wasm);
        parse_ast(node.child(1), &mut loop_block, is_wasm);
        block.define_while_loop(loop_block, conditional_block);
    } else if title == "id_asign" {
        let operator = node.child(1).get_value().unwrap();
        let var_name = node.child(0).get_value().unwrap();
        if operator == ":" {
            parse_ast(node.child(2), block, is_wasm);
        } else if operator == "+:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block, is_wasm);
            block.binop(BINOP::ADD);
        } else if operator == "-:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block, is_wasm);
            block.binop(BINOP::SUBTRACT);
        } else if operator == "*:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block, is_wasm);
            block.binop(BINOP::MULTIPLY);
        } else if operator == "/:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block, is_wasm);
            block.binop(BINOP::DIVIDE);
        } else if operator == "++" {
            block.load_variable(var_name);
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, "1");
            block.binop(BINOP::ADD);
        }
        else if operator == "--" {
            block.load_variable(var_name);
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, "1");
            block.binop(BINOP::SUBTRACT);
        }
        block.assign_variable(var_name);
    } else if title == "NUMBER" {
        let number_as_str = &node.get_value().unwrap();
        if number_as_str.contains(",") {
            block.load_const(
                celsium::BUILTIN_TYPES::FLOAT,
                &number_as_str.replace(",", "."),
            );
        } else {
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, &number_as_str);
        }
    } else if title == "STRING" {
        block.load_const(
            celsium::BUILTIN_TYPES::STRING,
            rem_first_and_last(node.get_value().unwrap()),
        );
    }
}
