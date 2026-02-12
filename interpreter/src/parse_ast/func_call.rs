use celsium::{ BuiltinTypes, block::Block, module::FuncArg };
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
        func_args_found.reverse();
        let func_name = node.child(0).get_value().unwrap();
        if func_name == "izvade" || func_name == "izvadetp" {
            block.push_to_testing_stack(true);
        }
        if func_name == "atgriezt" {
            block.return_from_function();
            if func_args_found.len() > 0 {
                compiler.typestack.push(func_args_found[0].clone());
            }
        } else {
            for std_function in celsium::std::get_std_functions() {
                if func_name == std_function.name {
                    check_function_signature(
                        compiler,
                        std_function.return_type,
                        std_function.args,
                        func_args_found.clone(),
                        node,
                        func_name.to_string()
                    );
                    block.call_special_function(func_name.to_string());
                    return;
                }
            }
            let func_id = util::lookup_function(
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

            check_function_signature(
                compiler,
                func_return_type,
                func_args,
                func_args_found,
                node,
                func_name.to_string()
            );

            block.call_function(func_name);
        }
    }
}

fn check_function_signature(
    compiler: &mut Compiler,
    return_type: Option<BuiltinTypes>,
    args: Vec<FuncArg>,
    args_found: Vec<BuiltinTypes>,
    node: AstNode,
    name: String
) {
    if return_type.is_some() {
        compiler.typestack.push(return_type.unwrap());
    }

    let arg_count_error = errors::CompileTimeErrorType::WrongFunctionArgumentCount {
        function_name: name.to_string(),
        expected_count: args.len(),
        found_count: args_found.len(),
    };

    if name == "izvade" || name == "izvadetp" || name == "garums" {
        if args_found.len() != 1 {
            compiler.add_error(arg_count_error.clone(), node);
        } else {
            return;
        }
    }
    //first check if argument cound is valid
    if args.len() != args_found.len() {
        compiler.add_error(arg_count_error, node);
    }
    //then check if arguement types are valid
    let mut counter = 0;
    for expected_arg in args {
        let found_arg = args_found[counter].clone();
        if expected_arg.arg_type != found_arg {
            compiler.add_error(
                errors::CompileTimeErrorType::WrongFunctionArgumentType {
                    function_name: name.to_string(),
                    arg_index: counter + 1,
                    expected_type: expected_arg.arg_type,
                    found_type: found_arg,
                },
                node
            );
        }
        counter += 1;
    }
}
