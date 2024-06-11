use std::thread::scope;

use celsium::{
    block::Block,
    compiletime_helper::CompileTimeHelper,
    module::{ FuncArg, FunctionSignature, VISIBILITY },
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::util;

use super::parse_ast;

pub fn func_def(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "func_def" {
        let func_name = node.child(0).get_value().unwrap().to_string();


        let mut body = Block::new(
            if node.children_count() > 2 {
                block.scope.change_ast_id(node.child(2).id())
            } else {
                block.scope.change_ast_id(node.child(1).id())
            }
        );
        let mut args: Vec<FuncArg> = vec![];

        if node.children_count() == 3 {
            //when the function takes arguments
            let args_tree = node.child(1).children();
            for arg in args_tree.iter().rev() {
                let arg_name = arg.child(1).get_value().unwrap().to_string();
                let arg_type =  util::data_type_from_str(arg.child(0).to_string().as_str());
                args.push(FuncArg {
                    name: arg_name.clone(),
                    arg_type: arg_type.clone()
                });
                let var_id = typestack.def_var(arg_name, arg_type, body.scope.clone());
                body.define_variable(var_id);
            }
            
            parse_ast(node.child(2), &mut body, is_wasm, typestack);
            typestack.def_function(func_name.clone(), args.clone(), block.scope.clone());
        } else {
            parse_ast(node.child(1), &mut body, is_wasm, typestack);
            typestack.def_function(func_name.clone(), args.clone(), block.scope.clone());
        }


        block.define_function(body, VISIBILITY::PUBLIC, FunctionSignature {
            name: func_name,
            return_type: celsium::module::FunctionReturnType::NONE,
            args: args,
        })
    }
}
