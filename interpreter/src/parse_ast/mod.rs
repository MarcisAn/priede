use core::panic;
use std::process::exit;
mod id_assign;
mod math_ops;
mod stumbrs;
use celsium::{
    block::Block,
    bytecode::BINOP,
    compile_time_checker::CompileTimeChecker,
    module::{FuncArg, FunctionSignature, VISIBILITY},
    BUILTIN_TYPES,
};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};
use id_assign::id_assign;
use stumbrs::*;

use crate::errors::{incorect_init_value, math_error};

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn parse_ast(
    node: AstNode,
    block: &mut Block,
    is_wasm: bool,
    typestack: &mut CompileTimeChecker,
) {
    let title = node.get_symbol().to_string();
    if title == "func_call" {
        if node.children_count() > 1 {
            for arg in node.child(1) {
                parse_ast(arg, block, is_wasm, typestack);
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
            parse_ast(node.child(1).child(0), block, is_wasm, typestack);
            parse_ast(node.child(1).child(1), block, is_wasm, typestack);
            block.call_special_function(celsium::SpecialFunctions::RANDOM);
        } else {
            block.call_function(func_name);
        }
    } else if title == "multiple_id_define" {
        stumbrs_define(node, block, typestack, is_wasm);
    } else if title == "return_st" {
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.return_from_function();
    } else if title == "var_def" {
        if node.child(0).get_symbol().to_string() == "ARRAY" {
            for i in node.child(2).children() {
                parse_ast(i, block, is_wasm, typestack);
            }
            block.define_array(
                celsium::module::VISIBILITY::PRIVATE,
                node.child(1).get_value().unwrap().to_string(),
                node.child(2).children().len(),
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
                incorect_init_value(
                    format!("Mainīgā datu tips ir norādīts kā `{}`, bet piešķirtā sākotnējā vērtība ir `{}`.",
                    match node.child(0).to_string().as_str() {
                        "NUM" => "skaitlis",
                        "BOOL_DEF" => "būls",
                        "TEXT" => "tekts",
                        "FLOAT" => "decimālskaitlis",
                        _ => panic!()
                    }, 
                    match typ_of_init_value.unwrap() {
                        BUILTIN_TYPES::MAGIC_INT => "skaitlis",
                        BUILTIN_TYPES::BOOL => "būls",
                        BUILTIN_TYPES::STRING => "teksts",
                        BUILTIN_TYPES::OBJECT => "objekts",
                        BUILTIN_TYPES::FLOAT => "decimālskaitlis",
                    } ),
                    &typestack.source_files[typestack.current_file],
                    &typestack.source_file_paths[typestack.current_file],
                    node.child(1).get_position().unwrap().line,
                    node.child(1).get_position().unwrap().column,
                );
                exit(0);
            }
            block.define_variable(
                data_type_marked,
                celsium::module::VISIBILITY::PRIVATE,
                node.child(1).get_value().unwrap(),
            )
        }
    } else if title == "func_def" {
        let mut body = Block::new();
        let mut args: Vec<FuncArg> = vec![];

        if node.children_count() > 2 {
            //when the function takes arguments
            parse_ast(node.child(2), &mut body, is_wasm, typestack);
            for arg in node.child(1).children() {
                args.push(FuncArg {
                    name: arg.child(1).get_value().unwrap().to_string(),
                    arg_type: match arg.child(0).to_string().as_str() {
                        "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                        "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                        "TEXT" => celsium::BUILTIN_TYPES::MAGIC_INT,
                        "FLOAT" => celsium::BUILTIN_TYPES::FLOAT,
                        _ => panic!(),
                    },
                })
            }
        } else {
            parse_ast(node.child(1), &mut body, is_wasm, typestack);
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
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.load_from_array(node.child(0).get_value().unwrap());
    } else if title.starts_with("ID") {
        block.load_variable(node.get_value().unwrap());
    } else if title == "plus"
        || title == "minus"
        || title == "reiz"
        || title == "dal"
        || title == "atlik"
    {
        math_ops::math_ops(
            match title.as_str() {
                "plus" => BINOP::ADD,
                "minus" => BINOP::SUBTRACT,
                "reiz" => BINOP::MULTIPLY,
                "dal" => BINOP::DIVIDE,
                "atlik" => BINOP::REMAINDER,
                _ => panic!(),
            },
            node,
            block,
            typestack,
            is_wasm,
        );
    } else if title == "if" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        let mut if_block = Block::new();
        parse_ast(node.child(1), &mut if_block, is_wasm, typestack);
        if node.children_count() > 2 {
            let mut else_block = Block::new();
            parse_ast(node.child(3), &mut else_block, is_wasm, typestack);
            block.define_if_else_block(if_block, else_block)
        } else {
            block.define_if_block(if_block);
        }
    } else if title == "comp_s" {
        let sign = node.child(1).get_value().unwrap();
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(2), block, is_wasm, typestack);
        let checked_type = match sign {
            "=" => typestack.binop(BINOP::EQ),
            ">" => typestack.binop(BINOP::LargerThan),
            ">=" => typestack.binop(BINOP::LargerOrEq),
            "<" => typestack.binop(BINOP::LessThan),
            "<=" => typestack.binop(BINOP::LessOrEq),
            "!=" => typestack.binop(BINOP::NotEq),
            _ => panic!("Neatpazīts salīdzinājuma simbols"),
        };
        if checked_type.is_none() {
            math_error(
                "Ar šiem datu tipiem nevar veikt šo matemātisko darbību",
                &typestack.source_files[typestack.current_file],
                &typestack.source_file_paths[typestack.current_file],
                node.child(0).get_position().unwrap().line,
                node.child(0).get_position().unwrap().column,
            );
            exit(0);
        }
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
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.binop(BINOP::AND);
    } else if title == "vai" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.binop(BINOP::OR);
    } else if title == "xvai" {
        parse_ast(node.child(0), block, is_wasm, typestack);
        parse_ast(node.child(1), block, is_wasm, typestack);
        block.binop(BINOP::XOR);
    } else if title == "block" {
        for i in node.children() {
            parse_ast(i, block, is_wasm, typestack);
        }
    } else if title == "s_loop" {
        let mut loop_block = Block::new();
        let mut loop_count_block = Block::new();

        parse_ast(node.child(1), &mut loop_block, is_wasm, typestack);
        parse_ast(node.child(0), &mut loop_count_block, is_wasm, typestack);
        block.define_simple_loop(loop_block, loop_count_block);
    } else if title == "w_loop" {
        let mut loop_block = Block::new();
        let mut conditional_block = Block::new();
        parse_ast(node.child(0), &mut conditional_block, is_wasm, typestack);
        parse_ast(node.child(1), &mut loop_block, is_wasm, typestack);
        block.define_while_loop(loop_block, conditional_block);
    } else if title == "id_assign" {
        id_assign(node, block, typestack, is_wasm);
    } else if title == "NUMBER" {
        let number_as_str = &node.get_value().unwrap();
        if number_as_str.contains(",") {
            block.load_const(
                celsium::BUILTIN_TYPES::FLOAT,
                &number_as_str.replace(",", "."),
            );
            typestack.push(BUILTIN_TYPES::FLOAT);
        } else {
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, &number_as_str);
            typestack.push(BUILTIN_TYPES::MAGIC_INT);
        }
    } else if title == "STRING" {
        block.load_const(
            celsium::BUILTIN_TYPES::STRING,
            rem_first_and_last(node.get_value().unwrap()),
        );
        typestack.push(BUILTIN_TYPES::STRING)
    } else if title == "BOOL" {
        block.load_const(
            BUILTIN_TYPES::BOOL,
            match node.child(0).to_string().as_str() {
                "TRUE" => "1",
                "FALSE" => "0",
                _ => panic!(),
            },
        );
        typestack.push(BUILTIN_TYPES::BOOL);
    }
}
