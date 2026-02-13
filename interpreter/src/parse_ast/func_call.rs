use celsium::{ BuiltinTypes, block::Block, module::FuncArg };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, util, Compiler };

use super::parse_ast;

pub fn func_call(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block, self_type: Option<BuiltinTypes>) {
    if title == "func_call" {
        let mut func_args_found: Vec<BuiltinTypes> = vec![];
        if self_type.is_some() {
            func_args_found.push(self_type.unwrap());
        }
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
                    compiler.check_function_signature(
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

            compiler.check_function_signature(
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

