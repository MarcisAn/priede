use std::thread::scope;

use celsium::{
    block::Block,
    compiletime_helper::CompileTimeHelper,
    module::{ FuncArg, FunctionSignature, VISIBILITY }, BUILTIN_TYPES,
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{errors, util};

use super::parse_ast;

pub fn func_def(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "func_def" {
        let is_exported = node.child(0).get_symbol().to_string() == "EXPORT";

        let func_name = node
            .child(0 + (is_exported as usize))
            .get_value()
            .unwrap()
            .to_string();

        let mut body = Block::new(
            if node.children_count() > 3 {
                block.scope.change_ast_id(node.child(3 + (is_exported as usize)).id())
            } else {
                block.scope.change_ast_id(node.child(2 + (is_exported as usize)).id())
            }
        );
        let mut args: Vec<FuncArg> = vec![];

        if node.children_count() == 4 {
            //when the function takes arguments
            let args_tree = node.child(1 + (is_exported as usize)).children();
            for arg in args_tree.iter().rev() {
                let arg_name = arg
                    .child(1 + (is_exported as usize))
                    .get_value()
                    .unwrap()
                    .to_string();




                let data_type_str = node
                    .child(0 + (is_exported as usize))
                    .get_value().unwrap();
                let data_type_marked_option = util::data_type_from_str(data_type_str);

                let mut data_type_marked: BUILTIN_TYPES;
                if data_type_marked_option.is_none() {
                    let struct_exists = typestack.struct_exists(data_type_str);
                    if struct_exists.is_some() {
                        data_type_marked = BUILTIN_TYPES::OBJECT {
                            name: struct_exists.clone().unwrap().name,
                            fields: struct_exists.unwrap().fields,
                        };
                    } else {
                        errors::notexistant_type(data_type_str.to_owned(), node, typestack);
                        panic!();
                    }
                }
                else{
                    data_type_marked = data_type_marked_option.unwrap();
                }


                args.push(FuncArg {
                    name: arg_name.clone(),
                    arg_type: data_type_marked.clone(),
                });
                let var_id = typestack.def_var(arg_name, data_type_marked, body.scope.clone(), is_exported);
                body.define_variable(var_id.unwrap());
            }

            parse_ast(node.child(3 + (is_exported as usize)), &mut body, is_wasm, typestack);
            typestack.def_function(
                func_name.clone(),
                args.clone(),
                block.scope.clone(),
                is_exported
            );
        } else {
            parse_ast(node.child(2 + (is_exported as usize)), &mut body, is_wasm, typestack);
            typestack.def_function(
                func_name.clone(),
                args.clone(),
                block.scope.clone(),
                is_exported
            );
        }

        block.define_function(body, VISIBILITY::PUBLIC, FunctionSignature {
            name: func_name,
            return_type: celsium::module::FunctionReturnType::NONE,
            args: args,
        })
    }
}
