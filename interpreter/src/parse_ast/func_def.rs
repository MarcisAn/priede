use std::thread::scope;

use celsium::{
    block::Block,
    compiletime_helper::CompileTimeHelper,
    module::{ FuncArg, FunctionSignature, VISIBILITY },
    BUILTIN_TYPES,
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ errors, util::{ self, get_data_type_from_id } };

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

        if node.children_count() >= 3 + (is_exported as usize) {
            //when the function takes arguments
            let mut return_type: Option<BUILTIN_TYPES> = None;

            let is_returning =
                node.child(2 + (is_exported as usize)).to_string() == "func_return_type";
            let args_tree = node.child(1 + (is_exported as usize)).children();
            for arg in args_tree.iter().rev() {
                let arg_name = arg
                    .child(1 + (is_exported as usize))
                    .get_value()
                    .unwrap()
                    .to_string();

                let data_type_str = arg.child(0).get_value().unwrap();

                let data_type_marked = get_data_type_from_id(typestack, data_type_str, node);

                args.push(FuncArg {
                    name: arg_name.clone(),
                    arg_type: data_type_marked.clone(),
                });
                let var_id = typestack.def_var(
                    arg_name,
                    data_type_marked,
                    body.scope.clone(),
                    is_exported
                );
                body.define_variable(var_id.unwrap());
            }

            if is_returning {
                return_type = Some(
                    util::get_data_type_from_id(
                        typestack,
                        node
                            .child(2 + (is_exported as usize))
                            .child(0)
                            .get_value()
                            .unwrap(),
                        node
                    )
                );
            }
            println!("{:?}", args);

            parse_ast(
                node.child(2 + (is_returning as usize) + (is_exported as usize)),
                &mut body,
                is_wasm,
                typestack
            );
            typestack.def_function(
                func_name.clone(),
                args.clone(),
                block.scope.clone(),
                is_exported,
                return_type
            );
        } else {
            let is_returning =
                node.child(1 + (is_exported as usize)).to_string() == "func_return_type";

            let mut return_type: Option<BUILTIN_TYPES> = None;
            if is_returning {
                return_type = Some(
                    util::get_data_type_from_id(
                        typestack,
                        node
                            .child(1 + (is_exported as usize))
                            .child(0)
                            .get_value()
                            .unwrap(),
                        node
                    )
                );
            }

            parse_ast(node.child(1 + (is_exported as usize)), &mut body, is_wasm, typestack);
            typestack.def_function(
                func_name.clone(),
                args.clone(),
                block.scope.clone(),
                is_exported,
                return_type
            );
        }

        block.define_function(body, VISIBILITY::PUBLIC, FunctionSignature {
            name: func_name,
            return_type: celsium::module::FunctionReturnType::NONE,
            args: args,
        });
    }
}
