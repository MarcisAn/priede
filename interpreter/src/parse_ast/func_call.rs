use celsium::{ block::Block, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, util, Compiler };

use super::parse_ast;

pub fn func_call(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "func_call" {
        let mut func_args_found: Vec<BuiltinTypes> = vec![];
        if node.children_count() > 1 {
            //if funccall has arguments
            for arg in node.child(1).children().iter() {
                parse_ast(arg, compiler, block);
                let arg_type = compiler.typestack.pop();
                if arg_type.is_some() {
                    func_args_found.push(arg_type.unwrap());
                }
            }
        }
        let func_name = node.child(0).get_value().unwrap();
        if func_name == "izvade" {
            block.push_to_testing_stack(true);
            block.call_special_function(celsium::SpecialFunctions::Print {
                newline: true,
            });
        } else if func_name == "atgriezt" {
            block.return_from_function();
            if func_args_found.len() > 0 {
                compiler.typestack.push(func_args_found[0].clone());
            }
        } else if func_name == "izvadetp" {
            block.call_special_function(celsium::SpecialFunctions::Print {
                newline: false,
            });
        } else if func_name == "ievade" {
            block.call_special_function(celsium::SpecialFunctions::Input);
            compiler.typestack.push(BuiltinTypes::String);
        } else if func_name == "jukums" {
            //parse_ast(node.child(1).child(0), block, is_wasm, typestack);
            //parse_ast(node.child(1).child(1), block, is_wasm, typestack);
            block.call_special_function(celsium::SpecialFunctions::Random);
            compiler.typestack.push(BuiltinTypes::Int);
        } else if func_name == "garums" {
            block.call_special_function(celsium::SpecialFunctions::Length);
            compiler.typestack.push(BuiltinTypes::Int);
        } else {
            let func_id = util::get_closest_scope(
                func_name.to_string(),
                block.scope.clone(),
                &mut compiler.helper,
                node
            );
            if func_id.is_none() {
                compiler.add_error(
                    errors::CompileTimeErrorType::FunctionNotDefined {
                        funcname: func_name.to_string(),
                    },
                    node
                );
            }
            let func_return_type = compiler.helper.get_func_return_type(func_id.unwrap()).unwrap();
            let mut func_args = compiler.helper.get_func_args(func_id.unwrap()).unwrap();
            func_args.reverse();
            func_args_found.reverse();

            //first check if argument cound is valid
            if func_args.len() != func_args_found.len() {
                compiler.add_error(
                    errors::CompileTimeErrorType::WrongFunctionArgumentCount {
                        function_name: func_name.to_string(),
                        expected_count: func_args.len(),
                        found_count: func_args_found.len(),
                    },
                    node
                );
            }
            //then check if arguement types are valid
            let mut counter = 0;
            for expected_arg in func_args {
                let found_arg = func_args_found[counter].clone();
                if expected_arg.arg_type != found_arg {
                    compiler.add_error(
                        errors::CompileTimeErrorType::WrongFunctionArgumentType {
                            function_name: func_name.to_string(),
                            arg_index: counter + 1,
                            expected_type: expected_arg.arg_type,
                            found_type: found_arg,
                        },
                        node
                    );
                }
                counter += 1;
            }

            if func_return_type.is_some() {
                compiler.typestack.push(func_return_type.unwrap());
            }
            block.call_function(func_name);
        }
    }
}
